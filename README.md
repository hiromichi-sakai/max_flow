# max flow in bipartite graph

## algorithm
| algorithm                  | running time(bipartite graph)         |
|----------------------------|---------------------------------------|
| FIFO push relabel          | O(n1 * m + n1^3)                      |
| Highest label push relabel | O(n1 * m + min(n1^3), n1^2 * sqrt(m)) |
| Dinic                      | O(n1^2 * m)                           |

## build
1. install Rust
    ```
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
2. install gcc
   ```
   yum install -y gcc
   ```
3. build
    ```
    cargo build --release
    ``` 

## run
```
./run.sh 
```

## result
### hilo
|#nodes|ratio|density| FIFO push relabel(ms) | highest label push relabel(ms) | dinic(ms) |
|--|--|--|-----------------------------|---------------------------------------|-----------|
|20000|5|2| 1050                        | 655                                   | 307       |
|20000|5|10| 4726                        | 1805                                  | 6338      |
|20000|1000|2| 8                           | 6                                     | 15        |
|20000|1000|10| 22                          | 14                                    | 30        |
|30000|5|2| 2564                        | 1725                                  | 857       |
|30000|5|10| 12466                       | 5323                                  | 18038     |
|30000|1000|2| 20                          | 15                                    | 40        |
|30000|1000|10| 86                          | 38                                    | 100       |
|40000|5|2| 4944                        | 2565                                  | 1372      |
|40000|5|10| 28341                       | 9193                                  | 36914     |
|40000|1000|2| 63                          | 22                                    | 62        |
|40000|1000|10| 136                         | 72                                    | 179       |

### rope
|#nodes|ratio|density| FIFO push relabel(ms) | highest label push relabel(ms) | dinic(ms) |
|--|--|--|--|--|--|
|20000|5|2|1281|664|66|
|20000|5|10|610|334|56|
|20000|1000|2|5|4|3|
|20000|1000|10|0|0|0|
|30000|5|2|3050|1546|149|
|30000|5|10|1770|944|164|
|30000|1000|2|13|11|6|
|30000|1000|10|13|7|2|
|40000|5|2|5736|3056|251|
|40000|5|10|3314|2153|367|
|40000|1000|2|34|31|16|
|40000|1000|10|17|16|11|

### zipf
|#nodes|ratio|density| FIFO push relabel(ms) | highest label push relabel(ms) | dinic(ms) |
|--|--|--|--|--|--|
|20000|5|2|18|50|3|
|20000|5|10|72|86|19|
|20000|1000|2|4|3|1|
|20000|1000|10|14|14|9|
|30000|5|2|16|99|6|
|30000|5|10|172|222|35|
|30000|1000|2|6|7|3|
|30000|1000|10|38|29|15|
|40000|5|2|25|164|5|
|40000|5|10|260|294|47|
|40000|1000|2|7|7|7|
|40000|1000|10|37|26|28|

## reference
* improved algorithms for bipartite network flow
* Solving Maximum Flow Problems on Real World Bipartite Graphs
