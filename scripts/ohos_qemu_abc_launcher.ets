export class OhosQemuLauncher {
  public invoke(abcPath: string, className: string, methodName: string): void {
    this.invokeRepeated(abcPath, className, methodName, 1);
  }

  public invokeRepeated(
    abcPath: string,
    className: string,
    methodName: string,
    iterations: int,
  ): void {
    try {
      const linker = new AbcRuntimeLinker(Class.of(this).getLinker(), [abcPath]);
      const targetClass = linker.loadClass(className);
      const targetMethod = targetClass.getStaticMethod(methodName);
      if (targetMethod === undefined) {
        throw new Error("static ABC entry method not found: " + methodName);
      }
      for (let iteration: int = 0; iteration < iterations; iteration++) {
        targetMethod.invoke();
      }
    } catch (error) {
      console.log("[QEMU ERROR] " + error);
      throw error;
    }
  }
}
