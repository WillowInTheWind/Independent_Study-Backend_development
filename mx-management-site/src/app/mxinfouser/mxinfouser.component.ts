import {Component, Inject} from '@angular/core';
import {
  MatExpansionPanel,
  MatExpansionPanelDescription,
  MatExpansionPanelHeader,
  MatExpansionPanelTitle
} from "@angular/material/expansion";
import {NgForOf, NgIf} from "@angular/common";
import {MorningExercise, MorningExService} from "../morning-ex.service";
import {MAT_DIALOG_DATA, MatDialogRef} from "@angular/material/dialog";
import {Observable} from "rxjs";

@Component({
  selector: 'app-mxinfouser',
  standalone: true,
  imports: [
    MatExpansionPanel,
    MatExpansionPanelDescription,
    MatExpansionPanelHeader,
    MatExpansionPanelTitle,
    NgForOf,
    NgIf
  ],
  templateUrl: './mxinfouser.component.html',
  styleUrl: './mxinfouser.component.css'
})
export class MxinfouserComponent {
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
  constructor(   public morningex: MorningExService, public dialogRef: MatDialogRef<MxinfouserComponent>,
                 @Inject(MAT_DIALOG_DATA) protected document: modal,) {
    document.mx.subscribe(mx => {
      this.mx = mx;
      this.mingrade = this.grades[mx.min_grade];
      this.maxgrade = this.grades[mx.max_grade]
      this.techreqs = mx.required_tech_json
    });
  }

  editMx(title: string) {
    // this.morningex.approveMx(title)
    window.location.reload()
  }
}

export interface modal {
  mx: Observable<MorningExercise>,
iseditor: boolean
}
