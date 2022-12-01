import path from "path";
import fs from "fs";

export async function main() {
    const typesFilePath = path.resolve(path.join(__dirname, "..", "src", "wrap", "types.ts"));
    let typesContent: string = fs.readFileSync(typesFilePath, "utf-8");
    typesContent = typesContent.replace(/interface EthereumProvider/g, "export interface EthereumProvider");
    typesContent = typesContent.replace(/export export/g, "export");
    fs.writeFileSync(typesFilePath, typesContent);

    const moduleFilePath = path.resolve(path.join(__dirname, "..", "src", "wrap", "module.ts"));
    let moduleContent: string = fs.readFileSync(moduleFilePath, "utf-8");
    moduleContent = moduleContent.replace("Client,\n", "");
    moduleContent = moduleContent.replace("MaybeAsync\n", "");
    fs.writeFileSync(moduleFilePath, moduleContent);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });