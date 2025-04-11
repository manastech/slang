import assert from "assert";
import fs from "node:fs";
import path from "path";
import * as solc06 from "solc06";
import * as solc07 from "solc07";
import * as solc08 from "solc08";
import * as solc089 from "solc089";
import * as solc0812 from "solc0812";
import * as solc0812 from "solc0823";

export function testFileSolC(languageVersion: string, dir: string, file: string) {
  var solc = undefined;
  if (languageVersion.startsWith("0.6")) {
    solc = solc06;
  }
  else if (languageVersion.startsWith("0.7")) {
    solc = solc07;
  }
  else if (languageVersion.startsWith("0.8.9")) {
    solc = solc089;
  }
  else if (languageVersion.startsWith("0.8.12")) {
    solc = solc0812;
  }
  else if (languageVersion.startsWith("0.8.23")) {
    solc = solc0823;
  }
  else if (languageVersion.startsWith("0.8")) {
    solc = solc08;
  }

  const start = performance.now();
  var folderMeta = `{
      "language": "Solidity",
      "sources": {
        "${file}": {
          "urls": ["${file}"]
        }
      },
      "settings": {
        "outputSelection": {
          "*": {
            "": ["ast"]
          }
        }
      }
    }
    `;
  const parsing_result = JSON.parse(solc.default.compile(folderMeta, { import: findImports(dir) }));

  assert(parsing_result["sources"] != undefined);
  if (parsing_result["errors"] && !parsing_result["errors"].every((value: any) => value["type"] == "Warning")) {
    console.log(parsing_result["errors"]);
    assert(false);
  }
}

function findImports(folder: string): (file: string) => { contents: string; } {
  const repoRoot = process.env["REPO_ROOT"];
  assert(repoRoot);
  return (file: string) => {
    const absolutePath = path.resolve(
      folder,
      file
    );
    const source = fs.readFileSync(absolutePath, "utf8");
    return { contents: source };
  };
}
