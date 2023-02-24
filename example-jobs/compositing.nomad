job "nogo-composite" {
  datacenters = ["dc1"]
  type = "batch"

  group "composition" {
    task "hexagon" {
      driver = "raw_exec"

      config {
        command = "nogo-draw"
      }

      env {
        NOGO_COLOR = "#FF0000FF"
        NOGO_OUTPUT = "/some/path/here/hexagon"
      }

      resources {
        cpu    = 500
        memory = 256
      }
    }

    task "square" {
      driver = "raw_exec"

      config {
        command = "nogo-draw"
      }

      env {
        NOGO_COLOR = "#00FF00FF"
        NOGO_SHAPE = "square"
        NOGO_OUTPUT = "/some/path/here/square"
      }

      resources {
        cpu    = 500
        memory = 256
      }
    }

    task "circle" {
      driver = "raw_exec"

      config {
        command = "nogo-draw"
      }

      env {
        NOGO_COLOR = "#0000FFFF"
        NOGO_SHAPE = "circle"
        NOGO_OUTPUT = "/some/path/here/circle"
      }

      resources {
        cpu    = 500
        memory = 256
      }
    }

    task "composite" {
      driver = "raw_exec"

      config {
        command = "nogo-composite"
      }

      env {
        NOGO_DIR = "/some/path/here"
        NOGO_OUTPUT = "/some/path/output"
      }

      lifecycle {
        hook = "poststop"
      }

      resources {
        cpu    = 500
        memory = 256
      }
    }
  }
}
