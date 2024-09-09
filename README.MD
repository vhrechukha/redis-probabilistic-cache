# Redis Eviction Policy Tester

## How to Run

1. `docker-compose up -d`
2. `cargo build`
3. `cargo run`

## Analysis of Results

1. `volatile-lru`: Evicts least recently used keys with TTL, effective in reducing memory usage. 
- Testing eviction policy: volatile-lru 
- Eviction policy set to: volatile-lru 
- Initial memory usage: 6305296 bytes 
- Initial key count: 21995 
- Final memory usage: 6315352 bytes 
- Final key count: 3830 
- Memory usage change: 10056 bytes 
- Key count change: -18165 

2. `allkeys-lru`: Evicts least recently used keys across all keys, moderate reduction in key count. 
- Testing eviction policy: allkeys-lru 
- Eviction policy set to: allkeys-lru 
- Initial memory usage: 6315352 bytes 
- Initial key count: 3830 
- Final memory usage: 6298504 bytes 
- Final key count: 382 
- Memory usage change: -16848 bytes 
- Key count change: -3448 

3. `volatile-lfu`: Evicts least frequently used keys with TTL, slight impact on memory usage.
- Testing eviction policy: volatile-lfu 
- Eviction policy set to: volatile-lfu 
- Initial memory usage: 6298504 bytes 
- Initial key count: 382 
- Final memory usage: 6297480 bytes 
- Final key count: 382 
- Memory usage change: -1024 bytes 
- Key count change: 0 

4. `allkeys-lfu`: Evicts least frequently used keys across all keys, varies depending on access patterns.
- Testing eviction policy: allkeys-lfu 
- Eviction policy set to: allkeys-lfu 
- Initial memory usage: 6297480 bytes 
- Initial key count: 382 
- Final memory usage: 6319008 bytes 
- Final key count: 382 
- Memory usage change: 21528 bytes 
- Key count change: 0 

5. `volatile-random`: Randomly evicts keys with TTL, moderate memory reduction.
- Testing eviction policy: volatile-random 
- Eviction policy set to: volatile-random 
- Initial memory usage: 6319008 bytes 
- Initial key count: 382 
- Final memory usage: 6298488 bytes 
- Final key count: 382 
- Memory usage change: -20520 bytes 
- Key count change: 0 

6. `allkeys-random`: Randomly evicts keys across all keys, minor memory changes.
- Testing eviction policy: allkeys-random 
- Eviction policy set to: allkeys-random 
- Initial memory usage: 6298488 bytes 
- Initial key count: 382 
- Final memory usage: 6297480 bytes 
- Final key count: 382 
- Memory usage change: -1008 bytes 
- Key count change: 0 

7. `volatile-ttl`: Evicts keys closest to expiration, may increase memory usage slightly due to key replacements.
- Testing eviction policy: volatile-ttl 
- Eviction policy set to: volatile-ttl 
- Initial memory usage: 6297480 bytes 
- Initial key count: 382 
- Final memory usage: 6298584 bytes 
- Final key count: 382 
- Memory usage change: 1104 bytes 
- Key count change: 0 

8. `noeviction`: Prevents any evictions, may result in OOM errors when memory is full.
- Testing eviction policy: noeviction 
- Eviction policy set to: noeviction 
- Initial memory usage: 6298584 bytes 
- Initial key count: 382 
- Encountered error when setting extra_key10000: OOM: command not allowed when used memory > 'maxmemory'. 
- Memory limit reached, stopping data insertion for this policy. 
- Final memory usage: 6298584 bytes 
- Final key count: 382 
- Memory usage change: 0 bytes 
- Key count change: 0 

## General Insights:
- **Eviction Effectiveness:** Policies like `volatile-lru` and `volatile-random` were more effective in reducing key count and freeing up memory. In contrast, `allkeys-lfu` actually saw an increase in memory usage, possibly due to less frequent evictions and more new keys being added.
- **Key Count Reductions:** The most significant reductions in key count occurred with policies like `volatile-lru` and `allkeys-lru`, which are designed to aggressively manage memory by evicting older or less frequently accessed keys.
