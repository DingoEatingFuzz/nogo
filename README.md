# Nogo

Logo, but it's for Nomad, sorta.

## What is it?

Nogo is a suite of batch tasks (functions, if you're fancy) and microservices (dare I say nanoservices, if you're fancy?) for making useless images in the style of the Logo (and the Logo turtle).

## Why would I want this?

You probably don't! It's meant to be an easy to grasp and easy to run collection of things to run in a Nomad cluster. Because making little pictures is more fun than tailing a task log that prints "Hello World".

## How do I use it?

First, you should try using `nogo-draw` on your local machine. It's a simple executable (also available as a Docker container) that reads from ENV to determine what to draw. Then it outputs a .png or a .svg file (depending on ENV).

Next you should try running this in a Nomad cluster. A sample job might take the form of something like this, but the point here is to give you stuff to play with, so don't take this as The Way, just a stating point, like this run-on sentence started off well enough and now it's just me riffing, a bit like that, just riff once you get it running.

```hcl
job "nogo" {
  type = "batch"

  task "redis" {
    driver = "docker"

    config {
      image = "dingoeatingfuzz/nogo-draw"
    }

    resources {
      cpu    = 500
      memory = 256
    }

    env {
      NOGO_SCRIPT = "forward 10; right 10;"
    }
  }
}
```

Perfect! That will generate an image.......inside the container that will be automatically destroyed. Well there's your first challenge: get the image out of the container before the container self-destructs (well techinically the client destructs the container at the request of the server, but that doesn't sounds as much like a spy extravaganza so please let me have this).

## And now what?

Got the image out? Nice, now you can play with some of the other Nogo utilities. Here's a fun one, `nogo-composite`. This one takes all the images in a directory and layers them on top of each other. How can you use this in a job spec? I dunno, maybe a `poststop` lifecyle hook?

## Okay, and now what??

The world is your oyster! Nogo is just simple bits of software to make playing with Nomad more fun. Here's a non-exhaustive list of things you can do next:

1. Use a parameterized job that accepts a Nogo script as the payload.
2. Make a Slackbot that accepts a script as a command and then shares the generated image in a thread upon completion.
3. Open an issue on this repo requesting a feature I'll never implement.
4. Regenerate images in Grafana using just the log output of the `nogo-draw` task. Can you do that? Probably, right? Grafana has everything.
5. Make a sysbatch job that runs `nogo-draw` on clients in an autoscaling group. Then scale that group up wicked high just to make pointless images in an extremely inefficient yet extremely satisfying way.
6. Use CSI to allow writes from a nogo job and then reads from some generic web server job so your images are automatically accessible from a web browser.
7. Experiment with one of a variety of task drivers. This is all writen in Rust, so maybe give WASM a go? Containers are a bit hefty for itsy bitsy binaries, maybe give Firecracker a go?

## Anything else?

Please clap.
