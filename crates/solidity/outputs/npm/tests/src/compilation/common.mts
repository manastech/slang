import path from "node:path";
import fs from "node:fs";
import assert from "node:assert";
import { CompilationBuilder } from "@nomicfoundation/slang/compilation";
import { readRepoFile } from "../utils/files.mjs";

const PATH = "crates/solidity/outputs/npm/tests/src/compilation/inputs";
export class SimpleGraph {
  nodes: string[];
  edges: Map<string, string[]>;

  public constructor() {
    this.nodes = [];
    this.edges = new Map();
  }

  public addNode(node: string) {
    if (!this.nodes.includes(node)) {
      this.nodes.push(node);
    }
    if (this.edges.get(node) == undefined) {
      this.edges.set(node, []);
    }
  }

  public addEdge(from: string, to: string) {
    this.addNode(from);
    this.addNode(to);
    const fromBucket = this.edges.get(from) || fail("addNode failed to add empty list");
    fromBucket.push(to);
  }

  public saveToDot(file: string) {
    fs.writeFileSync(file, "digraph {\n");
    this.edges.forEach((values, key) => {
      values.forEach((value) => fs.appendFileSync(file, `  "${key}" -> "${value}"\n`))
    });
    fs.appendFileSync(file, "}\n");
  }

  private isCyclicUtil(node: string, visited: Map<string, boolean>, recStack: Map<string, boolean>) {
    if (!visited.get(node)) {
      // Mark the current node as visited and part of recursion stack
      visited.set(node, true);
      recStack.set(node, true);

      // Recur for all the vertices adjacent to this node
      for (const x of this.edges.get(node) || []) {
        if (!visited.get(x) &&
          this.isCyclicUtil(x, visited, recStack)) {
          return true;
        } else if (recStack.get(x)) {
          return true;
        }
      }
    }

    // Remove the node from recursion stack
    recStack.set(node, false);
    return false;
  }

  public isCyclic() {
    const visited = new Map<string, boolean>();
    this.nodes.forEach((node) => visited.set(node, false));
    const recStack = new Map<string, boolean>();
    this.nodes.forEach((node) => visited.set(node, false));

    for (const node of this.nodes) {
      if (!visited.get(node) &&
        this.isCyclicUtil(node, visited, recStack)) {
        return true;
      }
    };

    return false;
  }

}

export async function createBuilder(imports: SimpleGraph = new SimpleGraph()): Promise<CompilationBuilder> {
  const builder = CompilationBuilder.create({
    languageVersion: "0.8.0",

    readFile: async (fileId) => {
      return readRepoFile(PATH, fileId);
    },

    resolveImport: async (sourceFileId, importPath) => {
      const importLiteral = importPath.node.unparse();
      assert(importLiteral.startsWith('"') || importLiteral.startsWith("'"));
      assert(importLiteral.endsWith('"') || importLiteral.endsWith("'"));

      const importString = importLiteral.slice(1, -1);

      let realSourceFile = path.join(PATH, sourceFileId);

      // HACK: The source file might be buried in some structure a/b/c/d/file.sol
      // in order to resolve its imports we allow ourselves to walk up the hierarchy
      // until we find the proper root of the import.
      let i = 0;
      while (i < 7) {
        let splat = Array(i + 1).fill("..");
        let file = path.join(sourceFileId, ...splat, importString);
        let realFile = path.join(PATH, file);
        if (fs.statSync(realFile, { throwIfNoEntry: false })) {
          imports.addEdge(realSourceFile.slice(PATH.length + 1), realFile.slice(PATH.length + 1));
          return file;
        }
        i++;
      }
      throw `Can't resolve import ${importPath}`;
    },
  });

  return builder;
}
