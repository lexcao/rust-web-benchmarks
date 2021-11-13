# Benchmarked On

## CPU

Intel(R) Core(TM) i7-8700 CPU @ 3.20GHz

Physical cores: 6

Logical cores: 12

## RAM

Type: DDR4

Size: 2 x 8 GB

Speed: 2667 MT/s

# Command

Executed [`hello-world.sh`](hello-world.sh) script.

# Frameworks

## Actix Web

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 10 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    1.53ms   3.17ms   0.04ms   98.15ms  
  Requests:
    Total: 3257559 Req/Sec: 327012.50
  Transfer:
    Total: 403.86 MB Transfer Rate: 40.54 MB/Sec
```

## Axum

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 10 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    1.74ms   1.68ms   0.04ms   206.74ms  
  Requests:
    Total: 2811752 Req/Sec: 282045.24
  Transfer:
    Total: 308.37 MB Transfer Rate: 30.93 MB/Sec
```

## Hyper

```
Benchmarking 500 connections @ http://127.0.0.1:3000 for 10 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    1.57ms   1.52ms   0.03ms   59.69ms  
  Requests:
    Total: 3111560 Req/Sec: 311971.45
  Transfer:
    Total: 264.10 MB Transfer Rate: 26.48 MB/Sec
```