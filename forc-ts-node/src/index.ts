
import { Command } from "commander";
import { ProgramTypeEnum } from "@fuel-ts/abi-typegen";
import { runTypegen } from "@fuel-ts/abi-typegen/runTypegen";

const program = new Command();

program
  .version("1.0.0")
  .description("An example forc plugin for generating Typescript types.")
  .requiredOption("-p, --path   [value]", "Path to the ABI json file")
  .option("-o, --output [value]", "Path to the output directory", "./types")
  .parse(process.argv);

const options = program.opts();

const cwd = process.cwd();
const inputs = [options.path];
const output = options.output;
const programType = ProgramTypeEnum.CONTRACT;

runTypegen({ cwd, inputs, output, programType });

console.log(`Generated types for ${options.path} in ${output} directory.`);