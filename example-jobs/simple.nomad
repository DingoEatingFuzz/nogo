job "nogo-simple" {
  datacenters = ["dc1"]
  type = "batch"

  task "nogo" {
    driver = "raw_exec"

    config {
      command = "nogo-draw"
    }

    env {
      NOGO_COLOR = "#000000FF"
      NOGO_BG_COLOR = "#FFFFFFFF"
      NOGO_OUTPUT = "/some/path/here/hexagon"
    }

    resources {
      cpu    = 500
      memory = 256
    }
  }
}
