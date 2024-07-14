include "root" {
  path = find_in_parent_folders()
}

include "common" {
  path = "${get_terragrunt_dir()}/../common.hcl"
  expose = true
}

generate "influxdb_provider" {
    path        = "influxdb_provider_override.tf"
    if_exists   = "overwrite"
    contents    = templatefile("${get_terragrunt_dir()}/../../terragrunt_templates/influxdb_provider.tpl", {
        url               = "http://${dependency.docker_server.outputs.ip}:${include.common.locals.config.influxdb.configuration_api.port}"
        username          = include.common.locals.config.influxdb.configuration_api.username
        password          = include.common.locals.config.influxdb.configuration_api.password
    })
}
