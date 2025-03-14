{
  runCommand,
  actionlint,
}:
runCommand "actionlint-check"
  {
    buildInputs = [
      actionlint
    ];
    src = ../.;
  }
  ''
    cp -r "$src"/. .
    mkdir -p $out
    actionlint .github/workflows/*
  ''
