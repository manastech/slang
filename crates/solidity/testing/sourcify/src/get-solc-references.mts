import * as fs from "fs";
import * as path from "path";
import assert from "node:assert";
import { promisify } from "node:util";

import * as solc from "solc";
import { isNodeType, findAll, astDereferencer, srcDecoder } from "solidity-ast/utils.js";
import type { SolcInput, SolcOutput } from "solidity-ast/solc.js";
import { Identifier, IdentifierPath, MemberAccess } from "solidity-ast";

function printReferences(input: SolcInput, output: SolcOutput) {
  const decoder = srcDecoder(input, output);
  const deref = astDereferencer(output);
  for (const source of Object.values(output.sources)) {
    const sourceUnit = source.ast;
    assert(isNodeType("SourceUnit", sourceUnit));

    for (const node of findAll(["Identifier", "IdentifierPath", "MemberAccess", "UsingForDirective"], sourceUnit)) {
      if (isNodeType("UsingForDirective", node)) {
        node.functionList?.forEach((fn: any) => {
          // not sure why this case is not handled properly by `findAll`, but it's not
          // this is using with user defined operators
          const identifierPath = fn.definition;
          if (identifierPath) {
            printReference(identifierPath);
          }
        });
      } else {
        printReference(node);
      }
    }
  }

  function printReference(node: Identifier | IdentifierPath | MemberAccess) {
    let reference;
    if (isNodeType("Identifier", node) || isNodeType("IdentifierPath", node)) {
      // `this` and `super` are keywords in Slang
      if (node.name === "this" || node.name === "super") return;

      reference = node.name;
    } else if (isNodeType("MemberAccess", node)) {
      reference = node.memberName;
    } else {
      return;
    }

    let definition;
    if (node.referencedDeclaration) {
      if (node.referencedDeclaration > 0x80000000) {
        definition = "built-in";
      } else {
        let refedDef = deref("*", node.referencedDeclaration);
        // definition = `${refedDef.nodeType} @ ${decoder(refedDef)}`;
        definition = `@ ${decoder(refedDef)}`;
      }
    } else {
      if (isNodeType("MemberAccess", node)) {
        // solc will not resolve built-in members
        definition = "built-in";
      } else {
        // this can happen in import deconstruction symbols, when there are
        // multiple overloads available
        definition = "ambiguous";
      }
    }

    // special case: `revert` is a statement and `type` is a special expression type in Slang
    if ((reference === "revert" || reference === "type") && definition === "built-in") return;

    // compute the reference location from the node, except for member
    // accesses, for which we want the location of the member itself
    let refLocation = node.src;
    if (isNodeType("MemberAccess", node)) {
      refLocation = node.memberLocation ?? node.src;
    }

    console.log(`'${reference}' @ ${decoder({ src: refLocation })} -> ${definition}`);
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
  const evmVersion = metadata.settings.evmVersion;
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
      evmVersion,
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
  console.error(
    "The path-to-project is the contract/project folder downloaded from Sourcify that contains the metadata.json file.",
  );
  process.exit(1);
}

try {
  const { input, compilerVersion } = buildSolcInput(projectPath);
  await loadRemoteVersion(`v${compilerVersion}`).then((snapshot) => {
    const output = JSON.parse(snapshot.compile(JSON.stringify(input), {}));
    const errors = output.errors?.filter((error: any) => error.severity === "error");
    if (errors?.length ?? 0 > 0) {
      errors.forEach((error: any) => console.error(error.formattedMessage));
      return;
    }
    printReferences(input, output);
  });
} catch (err: any) {
  console.error("Failed to read or compile contract: ", err.message);
  process.exit(1);
}
