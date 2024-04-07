import { Component } from '@angular/core';
import {MorningExercise, MorningExService} from "../morning-ex.service";

@Component({
  selector: 'app-mx-adminpage',
  standalone: true,
  imports: [],
  templateUrl: './mx-adminpage.component.html',
  styleUrl: './mx-adminpage.component.css'
})
export class MxAdminpageComponent {

  private morningExs: MorningExercise[];

  ngOnInit() {
    mx.get
  }
  constructor(
    private mx: MorningExService
  ) {
  }
}
