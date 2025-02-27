{
  inputs.utils.url = "github:numtide/flake-utils";

  outputs = { nixpkgs, utils, ... }:
    let mkShell = system: {
      devShells.default =
        let pkgs = import nixpkgs { system = system; config.allowUnfree = true; };
        in pkgs.mkShell rec {
          PGDATA = "/var/postgres/around";
          PGDATABASE = "postgres";
          PGHOST = "localhost";
          PGPORT = "54320";

          DATABASE_URL = "postgres://${PGHOST}:${PGPORT}/${PGDATABASE}";

          packages = [
            pkgs.awscli2
            pkgs.cargo
            pkgs.diesel-cli
            pkgs.just
            pkgs.postgresql
            pkgs.rust-analyzer
            pkgs.rustc
            pkgs.terraform
            pkgs.terraform-ls
          ];
        };
    };
    in utils.lib.eachDefaultSystem mkShell;
}
