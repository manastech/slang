import * as fs from "fs";
import * as path from "path";
import assert from "node:assert";
import { promisify } from "node:util";

import * as solc from "solc";
import { isNodeType, findAll, astDereferencer, srcDecoder } from "solidity-ast/utils.js";
import type { SolcInput, SolcOutput } from "solidity-ast/solc.js";

function printReferences(input: SolcInput, output: SolcOutput) {
  const decoder = srcDecoder(input, output);
  const deref = astDereferencer(output);
  for (const source of Object.values(output.sources)) {
    const sourceUnit = source.ast;
    assert(isNodeType("SourceUnit", sourceUnit));

    for (const ident of findAll(["Identifier", "IdentifierPath"], sourceUnit)) {
      if (ident.name === "this" || ident.name === "super") continue;

      const reference = `'${ident.name}' @ ${decoder(ident)}`;
      let definition;
      if (ident.referencedDeclaration) {
        if (ident.referencedDeclaration > 0x80000000) {
          definition = "built-in";
        } else {
          let refedDef = deref("*", ident.referencedDeclaration);
          definition = `${refedDef.nodeType} @ ${decoder(refedDef)}`;
        }
      } else {
        definition = "unresolved";
      }
      console.log(`${reference} -> ${definition}`);
    }
  }
}

const loadRemoteVersion: (version: string) => Promise<{ compile: (input: string, options: any) => string }> = promisify(
  solc.default.loadRemoteVersion,
);

function buildSolcInput(projectPath: string): { input: SolcInput; compilerVersion: string } {
  const metadataPath = path.join(projectPath, "metadata.json");
  if (!fs.existsSync(metadataPath)) {
    throw new Error(`the file ${metadataPath} does not exist`);
  }

  const raw = fs.readFileSync(metadataPath, "utf8");
  const metadata = JSON.parse(raw);

  const compilerVersion = metadata.compiler.version;
  const remappings = metadata.settings.remappings;
  const sources = Object.fromEntries(
    Object.entries(metadata.sources).map(([name, source]: [string, any]) => {
      const contents = fs.readFileSync(path.join(projectPath, "sources", source.keccak256), "utf8");
      return [name, { content: contents }];
    }),
  );
  const input = {
    language: "Solidity",
    sources,
    settings: {
      remappings,
      outputSelection: {
        "*": {
          "": ["ast"],
        },
      },
    },
  };
  return { input, compilerVersion };
}

const projectPath = process.argv[2];

if (!projectPath) {
  console.error("Usage: tsx get-solc-references.mts <path-to-project>");
  console.error("The path-to-project is the contract/project folder downloaded from Sourcify that contains the metadata.json file.");
  process.exit(1);
}

try {
  const { input, compilerVersion } = buildSolcInput(projectPath);
  await loadRemoteVersion(`v${compilerVersion}`).then((snapshot) => {
    const output = snapshot.compile(JSON.stringify(input), {});
    printReferences(input, JSON.parse(output));
  });
} catch (err: any) {
  console.error("Failed to read or compile contract: ", err.message);
  process.exit(1);
}
