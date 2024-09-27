#! /usr/bin/env bash

# run_test testdir testnumber
run_test () {
    local testdir=$1
    local testnum=$2
    local prefile=$testdir/$testnum.pre
    if [[ -f $prefile ]]; then
	eval $(cat $prefile)
    fi
    local testfile=$testdir/$testnum.run
    eval $(cat $testfile) > tests-out/$testnum.out 2> tests-out/$testnum.err
}

print_error_message () {
    local testnum=$1
    local contrunning=$2
    local filetype=$3
    builtin echo -e "\e[31mtest $testnum: standard $filetype incorrect\e[0m"
    echo "  what results should be found in file: $testdir/$testnum.$filetype"
    echo "  what results produced by your program: tests-out/$testnum.$filetype"
    echo "  compare the two using diff, cmp, or related tools to debug, e.g.:"
    echo "  prompt> diff $testdir/$testnum.$filetype tests-out/$testnum.$filetype"
    if (( $contrunning == 0 )); then
	exit 1
    fi
}

# check_test testdir testnumber contrunning out/err
check_test () {
    local testdir=$1
    local testnum=$2
    local contrunning=$3
    local filetype=$4

    # option to use cmp instead?
    returnval=$(diff $testdir/$testnum.$filetype tests-out/$testnum.$filetype)
    if (( $? != 0 )); then
	echo -n 1
    else
	echo -n 0
    fi
}

# run_and_check testdir testnumber contrunning verbose printerror
#   testnumber: the test to run and check
#   printerrer: if 1, print an error if test does not exist
run_and_check () {
    local testdir=$1
    local testnum=$2
    local contrunning=$3
    local verbose=$4
    local failmode=$5

    if [[ ! -f $testdir/$testnum.run ]]; then
	if (( $failmode == 1 )); then
	    echo "test $testnum does not exist" >&2; exit 1
	fi
	exit 0
    fi
    if (( $verbose == 1 )); then
	echo "running test $testnum"
	cat $testdir/$testnum.desc
    fi
    run_test $testdir $testnum
    outcheck=$(check_test $testdir $testnum $contrunning out)
    errcheck=$(check_test $testdir $testnum $contrunning err)
    # echo "results: outcheck:$outcheck errcheck:$errcheck"
    if (( $outcheck == 0 )) && (( $errcheck == 0 )); then
	builtin echo -e "\e[32mtest $testnum: passed\e[0m"
    else
	if (( $outcheck == 1 )); then
	    print_error_message $testnum $contrunning out
	fi
	if (( $errcheck == 1 )); then
	    print_error_message $testnum $contrunning err
	fi
    fi
}

# usage: call when args not parsed, or when help needed
usage () {
    echo "usage: run-tests.sh [-h] [-v] [-t test] [-c] [-d testdir]"
    echo "  -h                help message"
    echo "  -v                verbose"
    echo "  -t n              run only test n"
    echo "  -c                continue even after failure"
    echo "  -d testdir        run tests from testdir"
    return 0
}

#
# main program
#
verbose=0
testdir="tests"
contrunning=0
specific=""

args=`getopt hvct:d: $*`
if [[ $? != 0 ]]; then
    usage; exit 1
fi

set -- $args
for i; do
    case "$i" in
    -h)
	usage
	exit 0
        shift;;
    -v)
        verbose=1
        shift;;
    -c)
        contrunning=1
        shift;;
    -t)
        specific=$2
	shift
	number='^[0-9]+$'
	if ! [[ $specific =~ $number ]]; then
	    usage
	    echo "-t must be followed by a number" >&2; exit 1
	fi
        shift;;
    -d)
        testdir=$2
	shift
        shift;;
    --)
        shift; break;;
    esac
done

# need a test directory; must be named "tests-out"
if [[ ! -d tests-out ]]; then
    mkdir tests-out
fi

# run just one test
if [[ $specific != "" ]]; then
    run_and_check $testdir $specific $contrunning $verbose 1
    exit 0
fi

# run all tests
(( testnum = 1 ))
while true; do
    run_and_check $testdir $testnum $contrunning $verbose 0
    (( testnum = $testnum + 1 ))
done

exit 0










