import {Component, Inject} from '@angular/core';
import {
    MatExpansionPanel,
    MatExpansionPanelDescription,
    MatExpansionPanelHeader,
    MatExpansionPanelTitle
} from "@angular/material/expansion";
import {CommonModule, NgForOf, NgIf} from "@angular/common";
import {MorningExercise, MorningExService} from "../morning-ex.service";
import {MAT_DIALOG_DATA, MatDialogRef} from "@angular/material/dialog";
import {Observable} from "rxjs";
import {MatFormField, MatFormFieldModule, MatHint, MatLabel, MatSuffix} from "@angular/material/form-field";
import {MatOption} from "@angular/material/autocomplete";
import {MatSelect, MatSelectModule} from "@angular/material/select";
import {FormControl, FormGroup, ReactiveFormsModule, Validators} from "@angular/forms";
import {MatInput, MatInputModule} from "@angular/material/input";
import {
  MatDatepicker,
  MatDatepickerInput,
  MatDatepickerModule,
  MatDatepickerToggle
} from "@angular/material/datepicker";
import {MatCheckbox} from "@angular/material/checkbox";
import {Router, RouterOutlet} from "@angular/router";
import {provideNativeDateAdapter} from "@angular/material/core";

@Component({
  selector: 'app-editingmodal',
  standalone: true,
  imports: [
    MatDatepickerToggle,
    MatDatepicker,
    MatDatepickerInput,
    MatLabel,
    NgIf,
    CommonModule,
    MatFormField,
    MatSelect,
    MatOption,
    MatExpansionPanel,
    MatExpansionPanelHeader,
    MatExpansionPanelTitle,
    MatExpansionPanelDescription,
    ReactiveFormsModule,
    MatInput,
    MatSuffix,
    MatCheckbox
  ],
  providers: [
    provideNativeDateAdapter()
  ],
  templateUrl: './editingmodal.component.html',
  styleUrl: './editingmodal.component.css'
})
export class EditingmodalComponent {
  protected mx: MorningExercise;
  desc: string = "";

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
  ];
  mxdesc = [
    "Extension of an UPPER SCHOOL course or activity\n" ,
    "Extension of a MIDDLE SCHOOL course or activity\n" ,
    "Extension of an INTERMEDIATE SCHOOL course or activity\n" ,
    "Extension of a LOWER SCHOOL course or activity\n" ,
    "Student Musical Performance\n" ,
    "Outside Musical Perfromance\n" ,
    "Outside Speaker\n" ,
    "Student Theatre Performance\n" ,
    "Outside Theatre Performance"
  ];
  // @ts-ignore
  mx_details = new FormGroup({
    date:  new FormControl(new Date(), Validators.required),
    title: new FormControl('', Validators.required),
    description: new FormControl(this.desc, Validators.required),
    mingrade:  new FormControl('',{nonNullable: true}),
    maxgrade: new FormControl('', {nonNullable: true}),
    young_student_prep_instructions: new FormControl('', {nonNullable: true}),
    is_available_in_day: new FormControl('', {nonNullable: true}),
    Projector:  new FormControl('',{nonNullable: true}),
    PresentationSpecificLighting: new FormControl('', {nonNullable: true}),
    HandheldMics: new FormControl('', {nonNullable: true}),
    LavMics: new FormControl('', {nonNullable: true}),
    additionalreqs: new FormControl('', {nonNullable: true}),
    shortdescription:  new FormControl('',{nonNullable: true})
  });
  mingrade: string;
  maxgrade: string;
  techreqs: string[];
  constructor(   private route: Router,   public morningex: MorningExService, public dialogRef: MatDialogRef<EditingmodalComponent>,
                 @Inject(MAT_DIALOG_DATA) protected document: Observable<MorningExercise>,) {
    document.subscribe(mx => {
      this.mx = mx;
      this.mingrade = this.grades[mx.min_grade];
      this.desc = mx.description;
      this.maxgrade = this.grades[mx.max_grade]
      this.techreqs = mx.required_tech_json;

      for (let i of this.techreqs) {
        if (i == "Projector/Screen") {
          this.mx_details.patchValue({
            Projector: "true"
          });
        }
        else if (i == "Special presentation specific lighting") {
          this.mx_details.patchValue({
            PresentationSpecificLighting: "true"
          });
        }
        else if (i == "Additional handheld microphones") {
          this.mx_details.patchValue({
            HandheldMics: "true"
          });
        }
        else if (i == "On body Lav microphones") {
          this.mx_details.patchValue({
            LavMics: "true"
          });
        }
        else if (i !=  '' ) {
          this.mx_details.patchValue({
            additionalreqs: i
          });
        }

      }
      let day = new Date(this.mx.date)
      day.setDate(day.getDate() + 1)
      this.mx_details.patchValue({
        date: day,
        title: this.mx.title,
        description: this.mx.description,
        mingrade:  this.grades[this.mx.min_grade],
        maxgrade: this.grades[this.mx.max_grade],
        young_student_prep_instructions: this.mx.young_student_prep_instructions,
        additionalreqs: "",
        is_available_in_day: this.mx.is_available_in_day? "true": "false",
        shortdescription:  this.mx.short_description
      });
      console.log(this.mx_details.value.mingrade)
      console.log(this.techreqs)
    });
  }

  // saveMX(title: string) {
  //   this.morningex.editMx(title)
  //   window.location.reload()
  // }
  myFilter = (d: Date | null): boolean => {
    // @ts-ignore

    return (getCurrentDay(d) == 3 || getCurrentDay(d) == 7);
  };
  submitMX() {
    if (!this.mx_details.value.date) {return;}
    if (!this.mx_details.value.title || !this.mx_details.value.description ) {return}
    var required_tech_json: string = "";
    if (this.mx_details.value.Projector){
      required_tech_json += "Projector/Screen::";
    }
    if ( this.mx_details.value.PresentationSpecificLighting ){
      required_tech_json += "Special presentation specific lighting::";
    }
    if ( this.mx_details.value.HandheldMics ){
      required_tech_json += "Additional handheld microphones::";
    }
    if (    this.mx_details.value.LavMics ){
      required_tech_json += "On body Lav microphones::";
    }
    required_tech_json += this.mx_details.value.additionalreqs;
    // @ts-ignore
    var newdate = new Date(this.mx_details.value.date);
    var date = dateformat(newdate);
    var title: string = <string>this.mx_details.value.title;
    var description: string = <string>this.mx_details.value.description;
    // @ts-ignore
    var min_grade: number = this.grades.indexOf(<string>this.mx_details.value.mingrade);
    // @ts-ignore
    var max_grade: number =  this.grades.indexOf(<string>this.mx_details.value.maxgrade);
    var young_student_prep_instructions: string = <string>this.mx_details.value.young_student_prep_instructions;
    var is_available_in_day: boolean = <string>this.mx_details.value.is_available_in_day=="true";
    var short_description: string = <string>this.mx_details.value.shortdescription;
    var editors_json: string = "";
    var is_approved: boolean = false;
    console.log(this.mx.id)
    this.morningex.editMx(
      this.mx.id,
      date,
      title,
      description,
      min_grade,
      max_grade,
      young_student_prep_instructions,
      is_available_in_day,
      required_tech_json,
      short_description,
      editors_json,
      is_approved,
    )
    // this.basicdetailsform.reset()
    this.dialogRef.close()
    window.location.reload()
  }
}

var offdays = [["September 2 2024", "September 2 2024"], ["October 11 2024", "October 11 2024"], [ "October 14 2024", "October 14 2024"], [ "November 27 2024", "November 29 2024"], [ "December 20 2024", "January 3 2025"], [ "January 6 2025", "January 6 2025"], [ "January 17 2025", "January 17 2025"], [ "January 20 2025", "January 21 2025"], [ "February 17 2025", "February 21 2025"], [ "April 7 2025", "April 11 2025"], [ "April 14 2025", "April 14 2025"], [ "May 26 2025", "May 26 2025"]];

function getCurrentDay(today: Date) {
  let year = new Date().getFullYear();

  let start:  Date;
  if (new Date("1/22/25" + year.toString()).getTime() < new Date().getTime()) {
    start = new Date("1/21/25 23:59");
  } else {
    start = new Date("9/3/2024 23:59");
  }
  const offdays_parsed = offdays.map(e => {
      let startDate = new Date(e[0]+", 0:00")
      let endDate = new Date(e[1]+", 23:59")
      if (startDate.getMonth() > 7) {
        startDate.setFullYear(2024)
      } else {
        startDate.setFullYear(2025)
      }
      if (endDate.getMonth() > 7) {
        endDate.setFullYear(2024)
      } else {
        endDate.setFullYear(2025)
      }
      return [startDate, endDate]
    }
  )
  if (today <= new Date("9/4/24") || today >= new Date("6/5/25")|| offdays_parsed.some(e => e[0].getTime() <= today.getTime() && e[1].getTime() >= today.getTime()) || today.getDay() === 0 || today.getDay() === 6) {
    return 0;

  }
  let days = 0
  while (start.getTime() <= today.getTime()) {
    let day = start.getDay()
    if (day === 0 || day === 6 || offdays_parsed.some(e => e[0].getTime() <= start.getTime() && e[1].getTime() >= start.getTime()) ) {
      start.setDate(start.getDate() + 1)
      continue;
    }
    start.setDate(start.getDate() + 1)
    days += 1;
  }
  return days % 8;
}

function dateformat(date: Date): string {
  //
  if ((date.getMonth() - 10) < 0) {
    var month = "0" + (date.getMonth()+1).toString()
  }
  else{
    var month = (date.getMonth()+1).toString()
  }
  if (date.getDate() - 10 < 0) {
    var day = "0" + date.getDate().toString()
  }
  else{
    var day = date.getDate().toString()
  }
  return (date.getFullYear().toString() + "-" + month + "-" + day);


}
