terraform {
  required_providers {
    influxdb = {
      source = "DrFaust92/influxdb"
      version = "1.6.1"
    }
  }
}

provider "influxdb" {
  url      = "${url}"
  username = "${username}"
  password = "${password}"
  skip_ssl_verify = true
}