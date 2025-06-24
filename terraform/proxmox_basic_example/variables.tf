variable "vm_name" {
  description = "Name of the VM"
  type        = string
}

variable "vm_ip" {
  description = "Static IP address for the VM"
  type        = string
}

variable "gateway_ip" {
  description = "IP gateway"
  type        = string
}

variable "dns_ip" {
  description = "DNS IP"
  type        = string
}

variable "disk_size" {
  description = "Size of the disk in GB"
  type        = number
  default     = 32
}

variable "ram_size" {
  description = "Size of RAM in MB"
  type        = number
  default     = 2048
}

variable "cores" {
  description = "Number of cores for the CPU"
  type        = number
  default     = 2
}

variable "pm_api_url" {
  description = "Proxmox URL"
  type        = string
}

variable "pm_api_token_id" {
  description = "Proxmox API token ID"
  type        = string
  sensitive   = true
}

variable "pm_api_token_secret" {
  description = "Proxmox API token secret"
  type        = string
  sensitive   = true
}

variable "target_node" {
  description = "Proxmox node where the VM will be created"
  type        = string
}

variable "template_name" {
  description = "Base template to clone from"
  type        = string
}

variable "storage_name" {
  description = "Proxmox storage name to save the VM Disk"
  type        = string
}

variable "vm_user" {
  description = "Default username"
  type        = string
}

variable "ssh_key" {
  description = "SSH public key to inject into the VM"
  type        = string
}
