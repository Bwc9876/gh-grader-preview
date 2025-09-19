{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flakelight.url = "github:nix-community/flakelight";
    flakelight.inputs.nixpkgs.follows = "nixpkgs";
    flakelight-rust.url = "github:accelbread/flakelight-rust";
    flakelight-rust.inputs = {
      flakelight.follows = "flakelight";
    };
  };

  outputs = { flakelight-rust, ... }: flakelight-rust ./. { };
}
