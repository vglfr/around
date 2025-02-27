{
  inputs.utils.url = "github:numtide/flake-utils";

  outputs = { nixpkgs, utils, ... }:
    let mkShell = system: {
      devShells.default =
        let pkgs = nixpkgs.legacyPackages.${system};
        in pkgs.mkShell rec {
          PGDATA = "/var/postgres/around";
          PGDATABASE = "postgres";
          PGHOST = "localhost";
          PGPORT = "54320";

          DATABASE_URL = "postgres://${PGHOST}:${PGPORT}/${PGDATABASE}";

          packages = [
            pkgs.cargo
            pkgs.diesel-cli
            pkgs.just
            pkgs.postgresql
            pkgs.rust-analyzer
            pkgs.rustc
          ];
        };
    };
    in utils.lib.eachDefaultSystem mkShell;
}
