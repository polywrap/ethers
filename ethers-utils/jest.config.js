module.exports = {
  collectCoverage: true,
  preset: 'ts-jest',
  testEnvironment: 'node',
  testMatch: [
    "**/?(*.)+(spec|test).+(ts|tsx|js)",
    "/.polywrap/"
  ],
  transform: {
    "^.+\\.(ts|tsx)$": [
      "ts-jest", {
        tsconfig: "tsconfig.json",
        diagnostics: false
    }]
  },
};
