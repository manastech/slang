import path from "node:path";
import fs from "node:fs";
import assert from "node:assert";
import { CompilationBuilder } from "@nomicfoundation/slang/compilation";
import { readRepoFile } from "../utils/files.mjs";

export async function createBuilder(): Promise<CompilationBuilder> {
  const builder = CompilationBuilder.create({
    languageVersion: "0.8.0",

    readFile: async (fileId) => {
      return readRepoFile("crates/solidity/outputs/npm/tests/src/compilation/inputs", fileId);
    },

    resolveImport: async (sourceFileId, importPath) => {
      const importLiteral = importPath.node.unparse();
      assert(importLiteral.startsWith('"') || importLiteral.startsWith('\''));
      assert(importLiteral.endsWith('"') || importLiteral.endsWith('\''));

      const importString = importLiteral.slice(1, -1);

      // HACK: The source file might be buried in some structure a/b/c/d/file.sol
      // in order to resolve its imports we allow ourselves to walk up the hierarchy
      // until we find the proper root of the import.
      let i = 0;
      while (i < 5) {
        let splat = Array(i + 1).fill("..");
        let file = path.join(sourceFileId, ...splat, importString);
        let real_file = path.join("crates/solidity/outputs/npm/tests/src/compilation/inputs", file);
        if (fs.statSync(real_file, { throwIfNoEntry: false })) {
          return file;
        }
        i++;
      }
      throw `Can't resolve import ${importPath}`
    },
  });

  return builder;
}
