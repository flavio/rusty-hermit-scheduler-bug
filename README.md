# rusty hermit scheduler bug reproducer

This repository contains a simple application that connects against a Redis
database and perform some trivial operations against it.

The purpose is to show a weird scheduler bug (at least I think).

## Run

The demo application needs to interact with a Redis server. This can be
started with the help of docker:

```console
docker run --name some-redis --net host redis
```

> **Note:** the container will have access to the network stack of the
> host. This is convenient because it will make the Redis server
> reachable by the unikernel at the `10.0.2.2` address.

The bug takes place when a connection pool is being used, and the VM has just 1 vCPU.

This command shows that everything works fine when 1 vCPU is used, but no connection
pool is used:

```console
make run_single_core_no_pool
```

The following command shows that everything works fine when 4 vCPUs are assigned
to the VM and a connection pool is used:

```console
make run_multi_core_enable_pool
```

The next command will trigger the bug, since it runs a VM with just 1 CPU
and it also enables the connection pool:

```console
run_single_core_enable_pool
```

The application will be stuck for some seconds, then it will exit with an error
because the connection pool cannot establish a connection with the database.

## Possible root cause

I started to look into the code of the `redis` crate, then I went through
`r2d2` and I finally landed inside of [`parking_lot`](https://crates.io/crates/parking_lot).

When using the [`r2d2::Builder::build`](https://docs.rs/r2d2/0.8.10/r2d2/struct.Builder.html#method.build)
method, the internal [`wait_for_initialization`](https://github.com/sfackler/r2d2/blob/1178d1805dda0ec08f9cec626a67575691c0ce8f/src/lib.rs#L391-L404)
code is called.
This code relies on the [`parking_lot::Condvar`](https://docs.rs/parking_lot/latest/parking_lot/struct.Condvar.html)
structure, more specifically on the
[`Condvar::wait_until`](https://docs.rs/parking_lot/latest/parking_lot/struct.Condvar.html#method.wait_until)
method.

From my understanding, assuming we have have connection pool with just one connection,
a thread is going to initiate a connection towards the Redis server. At the same
time, another thread will be running the code that uses the `Condvar`. This thread
will be blocked until one of these two scenarios happen: either the connection
to the Redis database is active, or the `timeout` specified as parameter
of `Condvar` happens.

In my case, it turned out that the thread that attempts to connect to the Redis
server was never executed. Hence, after some time the `Condvar` timeout
kicked in and my `main` exited with an error.

After some time I realized I was running the unikernel using a VM that had
just 1 virtual CPU. Once I raised the number or VCPUs (2 was just fine), everything
went back to normal.

I assume the single VCPU was just blocked waiting for the `Condvar` condition
to be over? 🤷


