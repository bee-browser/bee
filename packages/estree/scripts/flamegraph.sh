SAVED=$(cat /proc/sys/kernel/perf_event_paranoid)
trap "echo $SAVED | sudo tee /proc/sys/kernel/perf_event_paranoid >/dev/null" EXIT

echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid >/dev/null
cat | cargo flamegraph --profile=profiling >/dev/null
