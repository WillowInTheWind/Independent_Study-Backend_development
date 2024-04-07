import { Routes } from '@angular/router';
import { AppComponent } from "./rootComponent/app.component";
import {LogincallbackComponent} from "./logincallback/logincallback.component";
import {HomepageComponent} from "./homepage/homepage.component";
import {MxAdminpageComponent} from "./mx-adminpage/mx-adminpage.component";

export const routes: Routes = [
  { path: '', component: HomepageComponent },
  { path: 'auth', component: LogincallbackComponent },
  { path: 'mxs', component: MxAdminpageComponent }

];
