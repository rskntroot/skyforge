# Skyforge

## Brief

Skyforge was designed to assist in render thousands of device configurations across the globe.

## Assumptions

- Partitions are groups of regions
- Regions are groups of zones
- Zones are groups of devices
- Layers are groups of common devices and facilitate template mapping

## Functionality

Skyforge takes a user provide regex pattern, performs a walk on a `./spec` dir, and matches a list of devices specifications that do not have the word "common" in their path.
 All group files are labeled with common and mappable from the file itself.

For each file_path matched, Skyforge then maps to all consituent files:

- Layer - from  by the common file in parent dir and maps the region
- Zonal - from the first group of chars in filename up to a `-` (region_id + zone_id)
- Regional - from the region_id of containing folder
- Partitional - from either layer (common.yaml) or regional yaml

Once all files are found, a compiled_specifcation is built.
 This spec is then passed to Tera as context.
 Tera then loads the template files for that layer and renders the configuration file.
