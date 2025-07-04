#!/bin/bash

# Default number of runs
N=${1:-100}

echo "Running benchmark.py $N times..."
echo

# Initialize arrays to store results
declare -a tiktoken_init_times=()
declare -a skimtoken_init_times=()
declare -a tiktoken_init_memories=()
declare -a skimtoken_init_memories=()
declare -a tiktoken_exec_times=()
declare -a skimtoken_exec_times=()
declare -a tiktoken_exec_memories=()
declare -a skimtoken_exec_memories=()
declare -a rmse_values=()

# Test run to debug
echo "Running test to check output parsing..."
test_output=$(uv run scripts/benchmark.py 2>&1)
echo "$test_output" | grep "Init Time" | head -1
echo

# Run benchmark N times
successful_runs=0
for i in $(seq 1 $N); do
    echo -ne "\rProgress: $i/$N (successful: $successful_runs)"
    
    # Run benchmark and capture output
    output=$(uv run scripts/benchmark.py 2>&1)
    
    # Extract RMSE value
    rmse=$(echo "$output" | grep -oE 'Mean RMSE: [0-9]+\.[0-9]+' | grep -oE '[0-9]+\.[0-9]+')
    
    # Extract values from the table
    # Init Time row
    init_time_line=$(echo "$output" | grep "│ Init Time")
    if [ ! -z "$init_time_line" ]; then
        # Extract numbers followed by 's' for seconds
        tiktoken_init_time=$(echo "$init_time_line" | sed -E 's/^[^0-9]*([0-9]+\.[0-9]+) s.*$/\1/' | grep -oE '^[0-9]+\.[0-9]+$')
        # Get the second occurrence of number followed by 's'
        skimtoken_init_time=$(echo "$init_time_line" | sed -E 's/^[^0-9]*[0-9]+\.[0-9]+ s[^0-9]*([0-9]+\.[0-9]+) s.*$/\1/' | grep -oE '^[0-9]+\.[0-9]+$')
    fi
    
    # Init Memory row
    init_mem_line=$(echo "$output" | grep "│ Init Memory")
    if [ ! -z "$init_mem_line" ]; then
        # Extract numbers followed by 'MB'
        tiktoken_init_mem=$(echo "$init_mem_line" | sed -E 's/^[^0-9]*([0-9]+\.[0-9]+) MB.*$/\1/' | grep -oE '^[0-9]+\.[0-9]+$')
        skimtoken_init_mem=$(echo "$init_mem_line" | sed -E 's/^[^0-9]*[0-9]+\.[0-9]+ MB[^0-9]*([0-9]+\.[0-9]+) MB.*$/\1/' | grep -oE '^[0-9]+\.[0-9]+$')
    fi
    
    # Exec Time row
    exec_time_line=$(echo "$output" | grep "│ Exec Time")
    if [ ! -z "$exec_time_line" ]; then
        tiktoken_exec_time=$(echo "$exec_time_line" | sed -E 's/^[^0-9]*([0-9]+\.[0-9]+) s.*$/\1/' | grep -oE '^[0-9]+\.[0-9]+$')
        skimtoken_exec_time=$(echo "$exec_time_line" | sed -E 's/^[^0-9]*[0-9]+\.[0-9]+ s[^0-9]*([0-9]+\.[0-9]+) s.*$/\1/' | grep -oE '^[0-9]+\.[0-9]+$')
    fi
    
    # Exec Memory row - handle potential negative values
    exec_mem_line=$(echo "$output" | grep "│ Exec Memory")
    if [ ! -z "$exec_mem_line" ]; then
        # Extract first number (may be negative)
        tiktoken_exec_mem=$(echo "$exec_mem_line" | perl -ne 'if (/│\s*Exec Memory\s*│\s*(-?[0-9]+\.[0-9]+)\s*MB/) { print $1 }')
        # Extract second number (may be negative)
        skimtoken_exec_mem=$(echo "$exec_mem_line" | perl -ne 'if (/│\s*Exec Memory\s*│\s*-?[0-9]+\.[0-9]+\s*MB\s*│\s*(-?[0-9]+\.[0-9]+)\s*MB/) { print $1 }')
    fi
    
    # Debug first run
    if [ $i -eq 1 ]; then
        echo -e "\n\nDebug - First run values:"
        echo "RMSE: $rmse"
        echo "tiktoken_init_time: $tiktoken_init_time"
        echo "skimtoken_init_time: $skimtoken_init_time"
        echo "tiktoken_init_mem: $tiktoken_init_mem"
        echo "skimtoken_init_mem: $skimtoken_init_mem"
        echo "tiktoken_exec_time: $tiktoken_exec_time"
        echo "skimtoken_exec_time: $skimtoken_exec_time"
        echo "tiktoken_exec_mem: $tiktoken_exec_mem"
        echo "skimtoken_exec_mem: $skimtoken_exec_mem"
        echo -e "\nContinuing...\n"
    fi
    
    # Add to arrays if we have the essential values (init time and memory are always positive)
    if [ ! -z "$tiktoken_init_time" ] && [ ! -z "$skimtoken_init_time" ] && \
       [ ! -z "$tiktoken_init_mem" ] && [ ! -z "$skimtoken_init_mem" ] && \
       [ ! -z "$tiktoken_exec_time" ] && [ ! -z "$skimtoken_exec_time" ]; then
        tiktoken_init_times+=($tiktoken_init_time)
        skimtoken_init_times+=($skimtoken_init_time)
        tiktoken_init_memories+=($tiktoken_init_mem)
        skimtoken_init_memories+=($skimtoken_init_mem)
        tiktoken_exec_times+=($tiktoken_exec_time)
        skimtoken_exec_times+=($skimtoken_exec_time)
        
        # Only add exec memory if both values exist (they might be negative)
        if [ ! -z "$tiktoken_exec_mem" ] && [ ! -z "$skimtoken_exec_mem" ]; then
            tiktoken_exec_memories+=($tiktoken_exec_mem)
            skimtoken_exec_memories+=($skimtoken_exec_mem)
        else
            # Use 0 if not found
            tiktoken_exec_memories+=(0)
            skimtoken_exec_memories+=(0)
        fi
        
        if [ ! -z "$rmse" ]; then
            rmse_values+=($rmse)
        fi
        
        successful_runs=$((successful_runs + 1))
    fi
done

echo -e "\n\nCalculating means..."

# Function to calculate mean
calculate_mean() {
    local arr=("$@")
    local sum=0
    local count=${#arr[@]}
    
    if [ $count -eq 0 ]; then
        echo "0"
        return
    fi
    
    for val in "${arr[@]}"; do
        sum=$(echo "$sum + $val" | bc -l)
    done
    
    echo "scale=6; $sum / $count" | bc -l
}

# Check if we have data
if [ ${#tiktoken_init_times[@]} -eq 0 ]; then
    echo "Error: No data was extracted from benchmark runs"
    echo "Please check that the benchmark.py script is working correctly"
    exit 1
fi

echo "Successfully extracted data from ${#tiktoken_init_times[@]} runs"

# Calculate means
mean_tiktoken_init_time=$(calculate_mean "${tiktoken_init_times[@]}")
mean_skimtoken_init_time=$(calculate_mean "${skimtoken_init_times[@]}")
mean_tiktoken_init_mem=$(calculate_mean "${tiktoken_init_memories[@]}")
mean_skimtoken_init_mem=$(calculate_mean "${skimtoken_init_memories[@]}")
mean_tiktoken_exec_time=$(calculate_mean "${tiktoken_exec_times[@]}")
mean_skimtoken_exec_time=$(calculate_mean "${skimtoken_exec_times[@]}")
mean_tiktoken_exec_mem=$(calculate_mean "${tiktoken_exec_memories[@]}")
mean_skimtoken_exec_mem=$(calculate_mean "${skimtoken_exec_memories[@]}")
mean_rmse=$(calculate_mean "${rmse_values[@]}")

# Calculate totals
mean_tiktoken_total_time=$(echo "$mean_tiktoken_init_time + $mean_tiktoken_exec_time" | bc -l)
mean_skimtoken_total_time=$(echo "$mean_skimtoken_init_time + $mean_skimtoken_exec_time" | bc -l)
mean_tiktoken_total_mem=$(echo "$mean_tiktoken_init_mem + $mean_tiktoken_exec_mem" | bc -l)
mean_skimtoken_total_mem=$(echo "$mean_skimtoken_init_mem + $mean_skimtoken_exec_mem" | bc -l)

# Calculate ratios
if (( $(echo "$mean_tiktoken_init_time > 0" | bc -l) )); then
    init_time_ratio=$(echo "scale=3; $mean_skimtoken_init_time / $mean_tiktoken_init_time" | bc -l)
else
    init_time_ratio="0"
fi

if (( $(echo "$mean_tiktoken_init_mem > 0" | bc -l) )); then
    init_mem_ratio=$(echo "scale=3; $mean_skimtoken_init_mem / $mean_tiktoken_init_mem" | bc -l)
else
    init_mem_ratio="0"
fi

if (( $(echo "$mean_tiktoken_exec_time > 0" | bc -l) )); then
    exec_time_ratio=$(echo "scale=3; $mean_skimtoken_exec_time / $mean_tiktoken_exec_time" | bc -l)
else
    exec_time_ratio="0"
fi

if (( $(echo "$mean_tiktoken_exec_mem > 0" | bc -l) )); then
    exec_mem_ratio=$(echo "scale=3; $mean_skimtoken_exec_mem / $mean_tiktoken_exec_mem" | bc -l)
else
    exec_mem_ratio="0"
fi

if (( $(echo "$mean_tiktoken_total_time > 0" | bc -l) )); then
    total_time_ratio=$(echo "scale=3; $mean_skimtoken_total_time / $mean_tiktoken_total_time" | bc -l)
else
    total_time_ratio="0"
fi

if (( $(echo "$mean_tiktoken_total_mem > 0" | bc -l) )); then
    total_mem_ratio=$(echo "scale=3; $mean_skimtoken_total_mem / $mean_tiktoken_total_mem" | bc -l)
else
    total_mem_ratio="0"
fi

# Display results
echo -e "\n╭────────────────── Mean Results After $successful_runs Runs ──────────────────╮"
printf "│ Mean RMSE: %.4f tokens                                        │\n" $mean_rmse
echo "├─────────────────┬──────────────┬──────────────┬────────────────┤"
echo "│ Metric          │   tiktoken   │  skimtoken   │     Ratio      │"
echo "├─────────────────┼──────────────┼──────────────┼────────────────┤"
printf "│ Init Time       │ %10.6f s │ %10.6f s │ %13.3fx │\n" $mean_tiktoken_init_time $mean_skimtoken_init_time $init_time_ratio
printf "│ Init Memory     │ %10.4f MB│ %10.4f MB│ %13.3fx │\n" $mean_tiktoken_init_mem $mean_skimtoken_init_mem $init_mem_ratio
printf "│ Exec Time       │ %10.6f s │ %10.6f s │ %13.3fx │\n" $mean_tiktoken_exec_time $mean_skimtoken_exec_time $exec_time_ratio
printf "│ Exec Memory     │ %10.4f MB│ %10.4f MB│ %13.3fx │\n" $mean_tiktoken_exec_mem $mean_skimtoken_exec_mem $exec_mem_ratio
echo "├─────────────────┼──────────────┼──────────────┼────────────────┤"
printf "│ TOTAL Time      │ %10.6f s │ %10.6f s │ %13.3fx │\n" $mean_tiktoken_total_time $mean_skimtoken_total_time $total_time_ratio
printf "│ TOTAL Memory    │ %10.4f MB│ %10.4f MB│ %13.3fx │\n" $mean_tiktoken_total_mem $mean_skimtoken_total_mem $total_mem_ratio
echo "╰─────────────────┴──────────────┴──────────────┴────────────────╯"