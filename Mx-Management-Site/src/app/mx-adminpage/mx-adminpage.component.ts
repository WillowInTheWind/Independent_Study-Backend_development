import { Component } from '@angular/core';
import {MorningExercise, MorningExService} from "../morning-ex.service";
import {Observable} from "rxjs";
import {AsyncPipe, CommonModule} from "@angular/common";

@Component({
  selector: 'app-mx-adminpage',
  standalone: true,
  imports: [
    AsyncPipe, CommonModule
  ],
  templateUrl: './mx-adminpage.component.html',
  styleUrl: './mx-adminpage.component.css'
})
export class MxAdminpageComponent {

  protected morningExs: Observable<MorningExercise[]> = this.mx.getallmxs();

  constructor(
    private mx: MorningExService
  ) {
  }
}
