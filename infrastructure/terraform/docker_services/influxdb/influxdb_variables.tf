variable "influxdb" {
  type = object({
    configuration_api = object({
      port      = number,
      username  = string
      password  = string
    })
    users = list(object({
        username    = string
        password    = string
        is_admin    = bool
        grants      = list(object({
            database    = string
            privilege   = string
        }))
    }))
    databases = list(object({
      name    = string
    }))
  })
  sensitive = true
  validation {
    condition     = alltrue([for user in var.influxdb.users : alltrue([for grant in user.grants : contains(["write", "read", "all"], lower(grant.privilege))])])
    error_message = "Each privilege must be one of 'write', 'read' or 'all'."
  }
}
