export class OhosQemuLauncher {
  public invoke(
    abcPath: string,
    className: string,
    methodName: string,
  ): void {
    try {
      const linker = new AbcRuntimeLinker(Class.of(this).getLinker(), [abcPath]);
      const targetClass = linker.loadClass(className);
      const targetMethod = targetClass.getStaticMethod(methodName);
      if (targetMethod === undefined) {
        throw new Error("static ABC entry method not found: " + methodName);
      }
      targetMethod.invoke();
    } catch (error) {
      console.log("[QEMU ERROR] " + error);
      throw error;
    }
  }
}
