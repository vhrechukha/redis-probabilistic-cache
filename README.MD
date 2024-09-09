# Redis Eviction Policy Tester

## How to Run

1. `docker-compose up -d`
2. `cargo build`
3. `cargo run`

## Analysis of Results

1. `volatile-lru`
Evicts least recently used keys with TTL, effective in reducing memory usage. <br>
<br>
Testing eviction policy: volatile-lru <br>
Eviction policy set to: volatile-lru <br>
Initial memory usage: 6305296 bytes <br>
Initial key count: 21995 <br>
Final memory usage: 6315352 bytes <br>
Final key count: 3830 <br>
Memory usage change: 10056 bytes <br>
Key count change: -18165 <br>

2. `allkeys-lru`
Evicts least recently used keys across all keys, moderate reduction in key count. <br>
<br>
Testing eviction policy: allkeys-lru <br>
Eviction policy set to: allkeys-lru <br>
Initial memory usage: 6315352 bytes <br>
Initial key count: 3830 <br>
Final memory usage: 6298504 bytes <br>
Final key count: 382 <br>
Memory usage change: -16848 bytes <br>
Key count change: -3448 <br>

3. `volatile-lfu`
Evicts least frequently used keys with TTL, slight impact on memory usage.<br>
<br>
Testing eviction policy: volatile-lfu <br>
Eviction policy set to: volatile-lfu <br>
Initial memory usage: 6298504 bytes <br>
Initial key count: 382 <br>
Final memory usage: 6297480 bytes <br>
Final key count: 382 <br>
Memory usage change: -1024 bytes <br>
Key count change: 0 <br>

4. `allkeys-lfu`
Evicts least frequently used keys across all keys, varies depending on access patterns.<br>
<br>
Testing eviction policy: allkeys-lfu <br>
Eviction policy set to: allkeys-lfu <br>
Initial memory usage: 6297480 bytes <br>
Initial key count: 382 <br>
Final memory usage: 6319008 bytes <br>
Final key count: 382 <br>
Memory usage change: 21528 bytes <br>
Key count change: 0 <br>

5. `volatile-random`
Randomly evicts keys with TTL, moderate memory reduction.<br>
<br>
Testing eviction policy: volatile-random <br>
Eviction policy set to: volatile-random <br>
Initial memory usage: 6319008 bytes <br>
Initial key count: 382 <br>
Final memory usage: 6298488 bytes <br>
Final key count: 382 <br>
Memory usage change: -20520 bytes <br>
Key count change: 0 <br>

6. `allkeys-random`
Randomly evicts keys across all keys, minor memory changes.<br>
<br>
Testing eviction policy: allkeys-random <br>
Eviction policy set to: allkeys-random <br>
Initial memory usage: 6298488 bytes <br>
Initial key count: 382 <br>
Final memory usage: 6297480 bytes <br>
Final key count: 382 <br>
Memory usage change: -1008 bytes <br>
Key count change: 0 <br>

7. `volatile-ttl`
Evicts keys closest to expiration, may increase memory usage slightly due to key replacements.<br>
<br>
Testing eviction policy: volatile-ttl <br>
Eviction policy set to: volatile-ttl <br>
Initial memory usage: 6297480 bytes <br>
Initial key count: 382 <br>
Final memory usage: 6298584 bytes <br>
Final key count: 382 <br>
Memory usage change: 1104 bytes <br>
Key count change: 0 <br>

8. `noeviction`
Prevents any evictions, may result in OOM errors when memory is full.<br>
<br>
Testing eviction policy: noeviction <br>
Eviction policy set to: noeviction <br>
Initial memory usage: 6298584 bytes <br>
Initial key count: 382 <br>
Encountered error when setting extra_key10000: OOM: command not allowed when used memory > 'maxmemory'. <br>
Memory limit reached, stopping data insertion for this policy. <br>
Final memory usage: 6298584 bytes <br>
Final key count: 382 <br>
Memory usage change: 0 bytes <br>
Key count change: 0 <br>

## General Insights:
- **Eviction Effectiveness:** Policies like `volatile-lru` and `volatile-random` were more effective in reducing key count and freeing up memory. In contrast, `allkeys-lfu` actually saw an increase in memory usage, possibly due to less frequent evictions and more new keys being added.
- **Key Count Reductions:** The most significant reductions in key count occurred with policies like `volatile-lru` and `allkeys-lru`, which are designed to aggressively manage memory by evicting older or less frequently accessed keys.
