locals {
  shapes = [
    { size = 0.75, weight = 4, rotation = 0, shape = "triangle" },
    { size = 0.75, weight = 4, rotation = 30, shape = "triangle" },
    { size = 0.5, weight = 10, rotation = 60, shape = "square" },
    { size = 0.75, weight = 4, rotation = 90, shape = "triangle" },
    { size = 0.75, weight = 4, rotation = 120, shape = "triangle" },
    { size = 0.75, weight = 4, rotation = 150, shape = "triangle" },
    { size = 0.75, weight = 4, rotation = 180, shape = "triangle" },
  ]
}

job "nogo-loop" {
  datacenters = ["dc1"]
  type = "batch"

  group "composition" {
    dynamic "task" {
      for_each = local.shapes
      labels = ["shape-${task.key}"]

      content {
        driver = "raw_exec"

        config {
          command = "nogo-draw"
        }

        env {
          NOGO_BG_COLOR = "#33336644"
          NOGO_OUTPUT = "/some/path/here/shape-${task.key}"
          NOGO_SHAPE = task.value.shape
          NOGO_ROTATION = task.value.rotation
          NOGO_WEIGHT = task.value.weight
          NOGO_SIZE = task.value.size
        }

        resources {
          cpu    = 500
          memory = 256
        }
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
