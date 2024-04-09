import { Component } from '@angular/core';
import {Observable} from "rxjs";
import {MorningExercise, MorningExService} from "../morning-ex.service";
import {AsyncPipe, CommonModule} from "@angular/common";

@Component({
  selector: 'app-user-mxs',
  standalone: true,
  imports: [
    AsyncPipe, CommonModule
  ],
  templateUrl: './user-mxs.component.html',
  styleUrl: './user-mxs.component.css'
})
export class UserMxsComponent {
  protected morningExs: Observable<MorningExercise[]> = this.mx.getusermxs();
  editMx(title: string) {
    // this.mx.editMx(title)
  }
  constructor(
    private mx: MorningExService
  ) {
  }
}
