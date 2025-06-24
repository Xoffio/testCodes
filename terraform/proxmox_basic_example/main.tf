# Specify required Terraform and provider versions
terraform {
  required_version = ">= 1.0.0" # Minimum version of Terraform CLI required
  required_providers {
    proxmox = {
      source  = "telmate/proxmox" # Use the Telmate Proxmox provider
      version = "3.0.2-rc01"
    }
  }
}

# Configure the Proxmox provider with API credentials and connection settings
provider "proxmox" {
  pm_api_url          = var.pm_api_url
  pm_api_token_id     = var.pm_api_token_id
  pm_api_token_secret = var.pm_api_token_secret
  pm_tls_insecure     = true
}

# Define the Proxmox VM resource
resource "proxmox_vm_qemu" "terra-test" {
  name        = var.vm_name       # Name of the new VM
  target_node = var.target_node   # Proxmox node where the VM will be created
  clone       = var.template_name # Base template to clone from
  full_clone  = true              # Create a full independent clone
  scsihw      = "virtio-scsi-single"
  os_type     = "cloud-init" # Enable Cloud-Init support

  memory = var.ram_size

  cpu {
    cores = var.cores
    type  = "host"
  }

  # Define disk layout using both SCSI and IDE (for Cloud-Init drive)
  disks {
    ide {
      ide2 {
        cloudinit {
          storage = var.storage_name # Attach Cloud-Init disk on ide2 using 'local' storage
        }
      }
    }
    scsi {
      scsi0 {
        disk {
          storage = var.storage_name # Main boot disk also on 'local' storage
          size    = var.disk_size    # Disk size in GB
        }
      }
    }
  }

  # Define virtual NIC configuration
  network {
    id        = 0
    model     = "virtio" # Use VirtIO network interface for performance
    bridge    = "vmbr0"  # Bridge interface to attach VM NIC to
    firewall  = false
    link_down = false # Ensure the NIC is up
  }

  # Attach a serial console for headless/Cloud-Init compatibility
  serial {
    id   = 0
    type = "socket"
  }

  # Cloud-Init user configuration
  ciuser     = var.vm_user                               # Default username for login
  ipconfig0  = "ip=${var.vm_ip}/24,gw=${var.gateway_ip}" # Static IPv4 network configuration
  nameserver = var.dns_ip                                # DNS resolver

  # Inject public SSH key for the ciuser
  sshkeys = var.ssh_key
}
