msg() {
    echo "configure: $1"
}

step_msg() {
    msg
    msg "$1"
    msg
}

warn() {
    echo "configure: WARNING: $1"
}

err() {
    echo "configure: error: $1"
    exit 1
}

need_ok() {
    if [ $? -ne 0 ]
    then
        err "$1"
    fi
}

need_cmd() {
    if command -v $1 >/dev/null 2>&1
    then msg "found $1"
    else err "need $1"
    fi
}

make_dir() {
    if [ ! -d $1 ]
    then
        msg "mkdir -p $1"
        mkdir -p $1
    fi
}

copy_if_changed() {
    if cmp -s $1 $2
    then
        msg "leaving $2 unchanged"
    else
        msg "cp $1 $2"
        cp -f $1 $2
        chmod u-w $2 # make copied artifact read-only
    fi
}

move_if_changed() {
    if cmp -s $1 $2
    then
        msg "leaving $2 unchanged"
    else
        msg "mv $1 $2"
        mv -f $1 $2
        chmod u-w $2 # make moved artifact read-only
    fi
}

putvar() {
    local T
    eval T=\$$1
    eval TLEN=\${#$1}
    if [ $TLEN -gt 35 ]
    then
        printf "configure: %-20s := %.35s ...\n" $1 "$T"
    else
        printf "configure: %-20s := %s %s\n" $1 "$T" "$2"
    fi
    printf "%-20s := %s\n" $1 "$T" >>config.tmp
}

probe() {
    local V=$1
    shift
    local P
    local T
    for P
    do
        T=$(command -v $P 2>&1)
        if [ $? -eq 0 ]
        then
            VER0=$($P --version 2>/dev/null | head -1 \
                |  sed -e 's/[^0-9]*\([vV]\?[0-9.]\+[^ ]*\).*/\1/' )
            if [ $? -eq 0 -a "x${VER0}" != "x" ]
            then
              VER="($VER0)"
            else
              VER=""
            fi
            break
        else
            VER=""
            T=""
        fi
    done
    eval $V=\$T
    putvar $V "$VER"
}

probe_need() {
    local V=$1
    probe $*
    eval VV=\$$V
    if [ -z "$VV" ]
    then
        err "needed, but unable to find any of: $*"
    fi
}

validate_opt () {
    for arg in $CFG_CONFIGURE_ARGS
    do
        isArgValid=0
        for option in $BOOL_OPTIONS
        do
            if test --disable-$option = $arg
            then
                isArgValid=1
            fi
            if test --enable-$option = $arg
            then
                isArgValid=1
            fi
        done
        for option in $VAL_OPTIONS
        do
            if echo "$arg" | grep -q -- "--$option="
            then
                isArgValid=1
            fi
        done
        if [ "$arg" = "--help" ]
        then
            echo
            echo "No more help available for Configure options,"
            echo "check the Wiki or join our IRC channel"
            break
        else
            if test $isArgValid -eq 0
            then
                err "Option '$arg' is not recognized"
            fi
        fi
    done
}

valopt() {
    VAL_OPTIONS="$VAL_OPTIONS $1"

    local OP=$1
    local DEFAULT=$2
    shift
    shift
    local DOC="$*"
    if [ $HELP -eq 0 ]
    then
        local UOP=$(echo $OP | tr '[:lower:]' '[:upper:]' | tr '\-' '\_')
        local V="CFG_${UOP}"
        eval $V="$DEFAULT"
        for arg in $CFG_CONFIGURE_ARGS
        do
            if echo "$arg" | grep -q -- "--$OP="
            then
                val=$(echo "$arg" | cut -f2 -d=)
                eval $V=$val
            fi
        done
        putvar $V
    else
        if [ -z "$DEFAULT" ]
        then
            DEFAULT="<none>"
        fi
        OP="${OP}=[${DEFAULT}]"
        printf "    --%-30s %s\n" "$OP" "$DOC"
    fi
}

opt() {
    BOOL_OPTIONS="$BOOL_OPTIONS $1"

    local OP=$1
    local DEFAULT=$2
    shift
    shift
    local DOC="$*"
    local FLAG=""

    if [ $DEFAULT -eq 0 ]
    then
        FLAG="enable"
    else
        FLAG="disable"
        DOC="don't $DOC"
    fi

    if [ $HELP -eq 0 ]
    then
        for arg in $CFG_CONFIGURE_ARGS
        do
            if [ "$arg" = "--${FLAG}-${OP}" ]
            then
                OP=$(echo $OP | tr 'a-z-' 'A-Z_')
                FLAG=$(echo $FLAG | tr 'a-z' 'A-Z')
                local V="CFG_${FLAG}_${OP}"
                eval $V=1
                putvar $V
            fi
        done
    else
        if [ ! -z "$META" ]
        then
            OP="$OP=<$META>"
        fi
        printf "    --%-30s %s\n" "$FLAG-$OP" "$DOC"
     fi
}

envopt() {
    local NAME=$1
    local V="CFG_${NAME}"
    eval VV=\$$V

    # If configure didn't set a value already, then check environment.
    #
    # (It is recommended that the configure script always check the
    # environment before setting any values to envopt variables; see
    # e.g.  how CFG_CC is handled, where it first checks `-z "$CC"`,
    # and issues msg if it ends up employing that provided value.)
    if [ -z "$VV" ]
    then
        eval $V=\$$NAME
        eval VV=\$$V
    fi

    # If script or environment provided a value, save it.
    if [ ! -z "$VV" ]
    then
        putvar $V
    fi
}
