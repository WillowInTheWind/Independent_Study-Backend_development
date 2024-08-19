import {bootstrapApplication, createApplication} from '@angular/platform-browser';
import { appConfig } from './app/app.config';
import { AppComponent } from './app/rootComponent/app.component';
import {ApplicationRef} from "@angular/core";

import {createCustomElement} from "@angular/elements";

bootstrapApplication(AppComponent, appConfig)
  .catch((err) => console.error(err));

