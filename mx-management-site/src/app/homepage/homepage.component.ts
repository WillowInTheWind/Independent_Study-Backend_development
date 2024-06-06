import { Component } from '@angular/core';
import {RouterOutlet} from "@angular/router";
import {MxFormComponent} from "../mx-form/mx-form.component";
import {MatCalendar} from "@angular/material/datepicker";
// import {MxFormComponent} from "../mx-form/mx-form.component";
// import {MatCalendar} from "@angular/material/datepicker";

@Component({
  selector: 'app-homepage',
  standalone: true,
  imports: [
    RouterOutlet,
    MxFormComponent,
    MatCalendar,
  ],
  templateUrl: './homepage.component.html',
  styleUrl: './homepage.component.css'
})
export class HomepageComponent {

}
