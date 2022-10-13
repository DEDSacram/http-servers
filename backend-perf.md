 

wrk http://127.0.0.1:8080/test -d 10 -t 1 -c 200

# Hello world response testing

time 10 s thread 1 concurrent connections 200

## Elixir/Cowboy
Thread Stats   Avg      Stdev     Max   +/- Stdev
Latency     6.23ms    3.60ms  53.98ms   93.28%
Req/Sec    33.76k     3.78k   37.03k    90.00%
335797 requests in 10.00s, 53.80MB read
Requests/sec:  33567.56
Transfer/sec:      5.38MB

## Nodejs/Fastify
Thread Stats   Avg      Stdev     Max   +/- Stdev
Latency     4.72ms    1.53ms  46.72ms   97.57%
Req/Sec    43.29k     5.38k   47.25k    96.00%
430200 requests in 10.00s, 80.00MB read
Requests/sec:  43005.25
Transfer/sec:      8.00MB

## Go/Fasthttp
Thread Stats   Avg      Stdev     Max   +/- Stdev
Latency     0.87ms  148.64us  10.28ms   95.87%
Req/Sec   116.27k     1.49k  119.13k    74.00%
1156631 requests in 10.00s, 166.56MB read
Requests/sec: 115614.46
Transfer/sec:     16.65MB

## Bun with siopao for routing
Thread Stats   Avg      Stdev     Max   +/- Stdev
Latency     0.87ms  232.28us   6.50ms   93.37%
Req/Sec   120.73k     1.69k  124.10k    87.00%
1200922 requests in 10.00s, 117.96MB read
Requests/sec: 120048.65
Transfer/sec:     11.79MB

## Rust/actix
Thread Stats   Avg      Stdev     Max   +/- Stdev
Latency   848.24us  174.08us   9.32ms   66.72%
Req/Sec   118.83k     8.28k  132.63k    44.00%
1182782 requests in 10.01s, 148.89MB read
Requests/sec: 118193.66
Transfer/sec:     14.88MB


## Rust/may-http
Thread Stats   Avg      Stdev     Max   +/- Stdev
Latency   826.52us  122.06us   1.82ms   59.63%
Req/Sec   121.67k     2.48k  129.22k    78.00%
1209972 requests in 10.01s, 141.93MB read
Requests/sec: 120916.57
Transfer/sec:     14.18MB

## Rust/warp
1 threads and 200 connections
Thread Stats   Avg      Stdev     Max   +/- Stdev
Latency   802.30us  117.46us   2.02ms   60.94%
Req/Sec   125.83k     2.78k  129.14k    94.00%
1251272 requests in 10.00s, 157.52MB read
Requests/sec: 125085.96
Transfer/sec:     15.75MB






