window.process = { env: { NODE_ENV: "development" } };
import { hUdemyApp } from "./components/hudemy-app";
import { hUdemyCoursesDrawer } from "./components/hudemy-courses-drawer";
import { hUdemyCourseDetail } from "./components/hudemy-course-detail";
import { hUdemyModule } from "./components/hudemy-module";

console.log("jfasdjflk");

customElements.define("hudemy-course-detail", hUdemyCourseDetail);
customElements.define("hudemy-courses-drawer", hUdemyCoursesDrawer);
customElements.define("hudemy-app", hUdemyApp);
customElements.define("hudemy-module", hUdemyModule);
