import { Routes } from '@angular/router';
import { AppComponent } from "./rootComponent/app.component";
import {LogincallbackComponent} from "./logincallback/logincallback.component";
import {HomepageComponent} from "./homepage/homepage.component";
import {MxAdminpageComponent} from "./mx-adminpage/mx-adminpage.component";
import {UserpageComponent} from "./userpage/userpage.component";
import {UserMxsComponent} from "./user-mxs/user-mxs.component";

export const routes: Routes = [
  { path: 'home', component: HomepageComponent },
  { path: '', redirectTo: 'home', pathMatch: 'full' },
  { path: 'auth', component: LogincallbackComponent },
  { path: 'mxs', component: MxAdminpageComponent },
  { path: 'me', component: UserpageComponent },
  { path: 'me/mxs', component: UserMxsComponent },
  { path: 'me/settings', component: UserpageComponent },
  { path: ':user', component: UserpageComponent }

];

