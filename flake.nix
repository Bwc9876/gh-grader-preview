{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flakelight-rust.inputs.flakelight.inputs.nixpkgs.follows = "nixpkgs";
  inputs.flakelight-rust.url = "github:accelbread/flakelight-rust";
  outputs = {flakelight-rust, ...}: flakelight-rust ./. {};
}
