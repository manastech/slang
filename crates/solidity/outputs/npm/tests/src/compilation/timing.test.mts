import { TerminalKind } from "@nomicfoundation/slang/cst";
import { createBuilder } from "./common.mjs";
import { max, mean, round, std } from "mathjs";
import assert from "node:assert";
import * as solc from "solc";
import path from "node:path";
import fs from "node:fs";

// test("DoodledBears sanctuary", async () => {
//   await testFile("015E220901014BAE4f7e168925CD74e725e23692_DoodledBears.sol");
// });

test("DoodledBears sourcify", async () => {
  const file = `0x015E220901014BAE4f7e168925CD74e725e23692/sources/DoodledBears.sol`
  await testFile(file);
});

test("DoodledBears solc", async () => {
  await testFileSolC("0.8.11", ["0x015E220901014BAE4f7e168925CD74e725e23692", "sources"]);
});

// test("WeightedPool sanctuary", async () => {
//   await testFile("01abc00E86C7e258823b9a055Fd62cA6CF61a163_WeightedPool.sol");
// });

test("WeightedPool sourcify", async () => {
  await testFile("0x01abc00E86C7e258823b9a055Fd62cA6CF61a163/sources/contracts/pools/weighted/WeightedPool.sol");
});

test("WeightedPool solc", async () => {
  await testFileSolC("0.7.0", ["0x01abc00E86C7e258823b9a055Fd62cA6CF61a163", "sources", "contracts"]);
});

test("UiPoolDataProviderV2V3 sourcify", async () => {
  await testFile("0x00e50FAB64eBB37b87df06Aa46b8B35d5f1A4e1A/contracts/misc/UiPoolDataProviderV2V3.sol");
});

test("UiPoolDataProviderV2V3 solc", async () => {
  await testFileSolC("0.6.12", ["0x00e50FAB64eBB37b87df06Aa46b8B35d5f1A4e1A", "contracts"]);
});

// test("YaxisVotePower sanctuary", async () => {
//   await testFile("01fef0d5d6fd6b5701ae913cafb11ddaee982c9a_YaxisVotePower.sol");
// });

test("YaxisVotePower sourcify", async () => {
  await testFile("0x01fef0d5d6fd6b5701ae913cafb11ddaee982c9a/YaxisVotePower/contracts/governance/YaxisVotePower.sol");
});

test("YaxisVotePower solc", async () => {
  await testFileSolC("0.6.12", ["0x01fef0d5d6fd6b5701ae913cafb11ddaee982c9a", "YaxisVotePower"]);
});

test("Mooniswap sourcify", async () => {
  await testFile("0x01a11a5A999E57E1B177AA2fF7fEA957605adA2b/sources/Users/k06a/Projects/mooniswap-v2/contracts/Mooniswap.sol");
});

test("Mooniswap solc", async () => {
  await testFileSolC("0.6.0", ["0x01a11a5A999E57E1B177AA2fF7fEA957605adA2b", "sources"]);
});

test("Darts sourcify", async () => {
  await testFile("0x01a5E3268E3987f0EE5e6Eb12fe63fa2AF992D83/sources/contracts/Darts.sol");
});

test("Darts solc", async () => {
  await testFileSolC("0.8.0", ["0x01a5E3268E3987f0EE5e6Eb12fe63fa2AF992D83", "sources"]);
});

test("ERC721AContract sourcify", async () => {
  await testFile("0x01665987bC6725070e56d160d75AA19d8B73273e/sources/project:/contracts/ERC721AContract.sol");
});

test("ERC721AContract solc", async () => {
  await testFileSolC("0.8.9", ["0x01665987bC6725070e56d160d75AA19d8B73273e", "sources"]);
});

test("SeniorBond sourcify", async () => {
  await testFile("0x0170f38fa8df1440521c8b8520BaAd0CdA132E82/sources/contracts/SeniorBond.sol");
});

test("SeniorBond solc", async () => {
  await testFileSolC("0.7.6", ["0x0170f38fa8df1440521c8b8520BaAd0CdA132E82", "sources"]);
});

function findImports(folder: string[]): (file: string) => { contents: string } {
  const repoRoot = process.env["REPO_ROOT"];
  assert(repoRoot);
  return (file: string) => {
    const absolutePath = path.resolve(repoRoot, "crates/solidity/outputs/npm/tests/src/compilation/inputs", ...folder, file);
    const source = fs.readFileSync(absolutePath, 'utf8');
    return { contents: source };
  }
}

async function testFileSolC(version: string, folder: string[]) {
  if (!version.startsWith("0.7")) {
    console.log(`Not processing ${folder[0]} (requires solc version ${version})`);
    return;
  }

  const repoRoot = process.env["REPO_ROOT"];
  assert(repoRoot);

  const start = performance.now();
  const folderPath = path.resolve(repoRoot, "crates/solidity/outputs/npm/tests/src/compilation/inputs", ...folder, "meta.json");
  var folderMeta = fs.readFileSync(folderPath, "utf8");
  const _value = JSON.parse(solc.default.compile(folderMeta, { import: findImports(folder) }));
  console.log(`Processing ${folder[0]} with solc takes ${round(performance.now() - start)}ms`);
  assert(_value["sources"] != undefined);
  if (_value["errors"] && !_value["errors"].every((value: any) => value["type"] == "Warning")) {

    console.log(_value["errors"]);
    assert(false);
  }
}

async function testFile(file: string) {
  let gotoDefTimes: number[] = Array();
  const startTime = performance.now();
  const builder = await createBuilder();

  await builder.addFile(file);

  const unit = builder.build();
  const cursor = unit.file(file)!.createTreeCursor();
  const setupTime = round(performance.now() - startTime);


  let neitherDefNorRef = 0;
  let defs = 0;
  let refs = 0;
  let ambiguousRefs = 0;
  let emptyRef = 0;

  // first access
  assert(typeof unit.bindingGraph.definitionAt == "function");
  const buildGraphTime = round(performance.now() - startTime - setupTime);

  while (cursor.goToNextTerminalWithKind(TerminalKind.Identifier)) {
    const startDefRef = performance.now();
    const definition = unit.bindingGraph.definitionAt(cursor);
    const reference = unit.bindingGraph.referenceAt(cursor);

    if (reference) {
      const defs = reference.definitions().length;
      if (defs > 1) {
        ambiguousRefs++;
      }
      else if (defs > 0) {
        refs++;
      }
      else {
        emptyRef++;
      }
    }

    const gotoDefTime = performance.now() - startDefRef;

    if (definition) {
      defs++;
    }

    const value = definition || reference;
    if (!value) {
      // console.log(`UNDEF: ${cursor.node}`);
      neitherDefNorRef += 1;
    }

    gotoDefTimes.push(gotoDefTime);
  }

  const totalTime = round(performance.now() - startTime);
  const resolutionTime = totalTime - buildGraphTime - setupTime;
  const maxGoto = round(max(gotoDefTimes));
  const meanGoto = round(mean(gotoDefTimes));
  const stdGoto = round(std(gotoDefTimes));
  console.log(`file: ${file}\n\trefs: ${refs}\tdefs: ${defs}\tneither: ${neitherDefNorRef}\tambiguous: ${ambiguousRefs}\tempty refs: ${emptyRef}\n\ttotal time: ${totalTime}ms\tsetup: ${setupTime}ms\tbuild: ${buildGraphTime}ms\tresolution: ${resolutionTime}ms\tmax: ${maxGoto}ms\tmean: ${meanGoto}ms\tstd: ${stdGoto}ms`);
}
