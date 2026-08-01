@Entry
@Component
struct Index {
  @State result: string = "ANI-RS static ABC packaged";

  aboutToAppear(): void {
    // DevEco's Stage compiler builds the UI module while the ArkTS 1.2
    // `native function` ABC is packaged under resources/rawfile. The QEMU HAP
    // runner extracts and executes that ABC with its matching ANI library.
    console.info("[ANI_RS_HAP_READY] static ABC and ANI library packaged");
  }

  build() {
    Column() {
      Text(this.result).fontSize(24)
    }
    .width("100%")
    .height("100%")
    .justifyContent(FlexAlign.Center);
  }
}
