import { Component } from '@angular/core';
import {RouterOutlet} from "@angular/router";
import {MxFormComponent} from "../mx-form/mx-form.component";

@Component({
  selector: 'app-homepage',
  standalone: true,
  imports: [
    RouterOutlet,
    MxFormComponent
  ],
  templateUrl: './homepage.component.html',
  styleUrl: './homepage.component.css'
})
export class HomepageComponent {

}
