# skyforge

## Brief

skyforge was designed to assist in rendering thousands of device configurations across the globe

## Assumptions

- Partitions are groups of regions
- Regions are groups of zones
- Zones are groups of devices
- Layers are groups of common devices and facilitate template mapping

## Functionality

Skyforge takes a user provided regex pattern, performs a walk on a `./spec` dir,
 and matches a list of devices specifications that do not have the word "common" in their path.
 All group files are labeled with common and mappable from the file itself.

For each "device" matched, skyforge then maps to all consituent files:

- Layer - from the `common.yaml` file in parent dir and maps the region
- Zonal - from the first group of chars in filename up to a `-` which is expected to be region and zone
- Regional - from the `<region>/common/<network>.yaml` of containing folder where network matches the layer info
- Partitional - from either layer (common.yaml) or regional yaml

Once all files are found, a compiled specifcation is built.
 This spec is then passed to Tera as context.
 Tera then loads the template files for that layer and renders the configuration files.

## Usage

from the `skyforge/demo` directory

### Help

``` bash
Skyforge Config Generation Engine

Usage: skyforge [OPTIONS] --devices <devices>

Options:
  -d, --devices <devices>  A regular expression pattern
      --debug              Print debug information
  -v, --verbose            Print verbose information
  -h, --help               Print help
  -V, --version            Print version

Environment:
    SF_SPEC_PATH    Path to the directory containing templates.        Defaults to "./spec".
    SF_TMPL_PATH    Path to the directory containing specifications.   Defaults to "./tmpl".
    SF_OUT_PATH     Path to the directory for command output.          Defaults to "./out".
    SF_LOG_PATH     Path to the directory for log output.              Defaults to "./log".
```

### Standard

``` bash
$ skyforge -d xyz1-ex-edge-r101
Skyforge found 8 renderable devices in /home/lost/workspace/skyforge/demo
Matched 1 devices against 'xyz1-ex-edge-r101'
Rendering xyz1-ex-edge-r101
Writing Output
 | out/xyz1-ex-edge-r101/all.conf
```

### Verbose

``` bash
$ skyforge -d xyz1-ex-edge-r101 -v
Skyforge found 8 renderable devices in /home/lost/workspace/skyforge/demo
Matched 1 devices against 'xyz1-ex-edge-r101'
 | ./spec/xyz/ex-edge-r1/xyz1-ex-edge-r101.yaml
Rendering xyz1-ex-edge-r101
 | ./tmpl/ex-edge-r/system.tmpl
 | ./tmpl/ex-edge-r/chassis.tmpl
 | ./tmpl/ex-edge-r/interfaces.tmpl
 | ./tmpl/ex-edge-r/protocols.tmpl
Writing Output
 | out/xyz1-ex-edge-r101/system.tmpl
 | out/xyz1-ex-edge-r101/chassis.tmpl
 | out/xyz1-ex-edge-r101/interfaces.tmpl
 | out/xyz1-ex-edge-r101/protocols.tmpl
 | out/xyz1-ex-edge-r101/compiled.spec
 | out/xyz1-ex-edge-r101/all.conf
```

### Debug

``` bash
$ skyforge -d xyz1-ex-edge-r101 --debug
devices: xyz1-ex-edge-r101, loglevel: Debug, env: spec_path: ./spec, tmpl_path: ./tmpl, out_path: ./out, log_path: ./log
Skyforge found 8 renderable devices in /home/lost/workspace/skyforge/demo
Matched 1 devices against 'xyz1-ex-edge-r101'
 | ./spec/xyz/ex-edge-r1/xyz1-ex-edge-r101.yaml
Compiled Spec for 'xyz1-ex-edge-r101.yaml'
 | ./spec/common/us.yaml
 | ./spec/xyz/common/ex.yaml
 | ./spec/xyz/ex-edge-r1/common.yaml
 | ./spec/xyz/ex-edge-r1/xyz1.common.yaml
 | ./spec/xyz/ex-edge-r1/xyz1-ex-edge-r101.yaml
Rendering xyz1-ex-edge-r101
 | ./tmpl/ex-edge-r/system.tmpl
 | ./tmpl/ex-edge-r/chassis.tmpl
 | ./tmpl/ex-edge-r/interfaces.tmpl
 | ./tmpl/ex-edge-r/protocols.tmpl
Writing Output
 | out/xyz1-ex-edge-r101/system.tmpl
 | out/xyz1-ex-edge-r101/chassis.tmpl
 | out/xyz1-ex-edge-r101/interfaces.tmpl
 | out/xyz1-ex-edge-r101/protocols.tmpl
 | out/xyz1-ex-edge-r101/compiled.spec
 | out/xyz1-ex-edge-r101/all.conf
```
