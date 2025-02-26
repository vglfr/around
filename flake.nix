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
          PGPORT = "5432";

          DATABASE_URL = "postgres://${PGHOST}:${PGPORT}/${PGDATABASE}";

          packages = [
            pkgs.cargo
            pkgs.diesel-cli
            pkgs.postgresql
            pkgs.rust-analyzer
            pkgs.rustc
          ];

          shellHook = ''
          '';
        };
    };
    in utils.lib.eachDefaultSystem mkShell;
}
