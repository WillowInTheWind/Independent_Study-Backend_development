import {Component, Inject} from '@angular/core';
import {MAT_DIALOG_DATA, MatDialogRef} from "@angular/material/dialog";
import {CookieService} from "ngx-cookie-service";
import {AsyncPipe, DOCUMENT, NgForOf, NgIf} from "@angular/common";
import {MorningExercise, MorningExService} from "../morning-ex.service";
import {Observable} from "rxjs";
import {
  MatExpansionPanel,
  MatExpansionPanelDescription,
  MatExpansionPanelHeader,
  MatExpansionPanelTitle
} from "@angular/material/expansion";

@Component({
  selector: 'app-mxinfo',
  standalone: true,
  imports: [
    AsyncPipe,
    MatExpansionPanel,
    MatExpansionPanelDescription,
    MatExpansionPanelHeader,
    MatExpansionPanelTitle,
    NgForOf,
    NgIf
  ],
  templateUrl: './mxinfo.component.html',
  styleUrl: './mxinfo.component.css'
})
export class MxinfoComponent {
  grades = [
    'JK',
    'SK',
    '1st',
    '2nd',
    '3rd',
    '4th',
    '5th',
    '6th',
    '7th',
    '8th',
    '9th',
    '10th',
    '11th',
    '12th'
  ]
  mingrade: string;
  maxgrade: string;
  techreqs: string[];
  protected mx: MorningExercise;
  constructor(   public morningex: MorningExService, public dialogRef: MatDialogRef<MxinfoComponent>,
                  @Inject(MAT_DIALOG_DATA) protected document: Observable<MorningExercise>,) {
    document.subscribe(mx => {
      this.mx = mx;
      this.mingrade = this.grades[mx.min_grade];
      this.maxgrade = this.grades[mx.max_grade]
      this.techreqs = mx.required_tech_json
    });
  }
  approveMx(title: string) {
    this.morningex.approveMx(title)
    window.location.reload()
  }
  revokeMx(title: string) {
    // this.morningex.approveMx(title)
    window.location.reload()
  }
}

