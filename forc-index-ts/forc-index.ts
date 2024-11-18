import * as path from "jsr:@std/path";
import { parse as parseToml } from "jsr:@std/toml";
import { Command } from "npm:commander@^12.1.0";
import process from "node:process";
import chalk from "npm:chalk@^5.3.0";

interface ForcConfig {
  project: {
    metadata?: {
      indexing?: {
        schema_path?: string;
      };
    };
  };
}

interface AbiLoggedType {
  logId: string;
  concreteTypeId: string;
}

interface AbiSchema {
  loggedTypes?: AbiLoggedType[];
}

const program = new Command();


program
  .version("1.0.0")
  .description("Forc.toml metadata parser that validates and reads ABI schema")
  .requiredOption("-p, --path <value>", "Path to the Forc.toml file")
  .parse(process.argv);

const options = program.opts();

try {
  // Read and validate Forc.toml
  console.log(chalk.blue.bold('\n📖 Reading Forc.toml configuration...'));
  const content = Deno.readFileSync(options.path);
  const contentString = new TextDecoder().decode(content);
  const forcConfig = parseToml(contentString) as unknown as ForcConfig;

  // Validate the required structure exists
  if (!forcConfig.project?.metadata?.indexing?.schema_path) {
    throw new Error(chalk.red('Missing required project.metadata.indexing.schema_path in Forc.toml'));
  }

  // Get the ABI schema path and read it
  const schemaPath = forcConfig.project.metadata!.indexing!.schema_path;
  const projectPath = path.dirname(options.path);
  const abiPath = path.join(projectPath, schemaPath);

  console.log(chalk.green('✓ Forc.toml validated successfully'));
  console.log(chalk.blue.bold('\n📑 Reading ABI schema...'));
  console.log(chalk.dim(`Path: ${abiPath}`));

  const abiContent = Deno.readFileSync(abiPath);
  const abiSchema = JSON.parse(new TextDecoder().decode(abiContent)) as AbiSchema;

  if (!abiSchema.loggedTypes || abiSchema.loggedTypes.length === 0) {
    console.log(chalk.yellow('⚠️  No logged types found in ABI'));
    process.exit(1);
  }

  console.log(chalk.green('✓ ABI schema loaded successfully'));
  console.log(chalk.magenta.bold('\n📋 Logged Types:'));
  
  abiSchema.loggedTypes!.forEach((loggedType, index) => {
    console.log(chalk.cyan(`\n[Entry ${index + 1}]`));
    console.log(chalk.yellow(`  🔍 Log ID: ${chalk.white(loggedType.logId)}`));
    console.log(chalk.yellow(`  🔑 Concrete Type ID: ${chalk.white(loggedType.concreteTypeId)}`));
  });

  console.log('\n'); // Add extra newline for cleaner output

} catch (error) {
  if (error instanceof Error) {
    console.error(chalk.red.bold('❌ Error: ') + chalk.red(error.message));
    process.exit(1);
  }
  throw error;
}
