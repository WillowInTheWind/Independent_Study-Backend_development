import { Component } from '@angular/core';
import {MorningExercise, MorningExService} from "../morning-ex.service";
import {min, Observable} from "rxjs";
import {AsyncPipe, CommonModule} from "@angular/common";
import {FormControl, FormGroup, FormsModule, ReactiveFormsModule} from "@angular/forms";
import {
  MatExpansionPanel,
  MatExpansionPanelDescription,
  MatExpansionPanelHeader,
  MatExpansionPanelTitle
} from "@angular/material/expansion";
import {MatDialog} from "@angular/material/dialog";
import {MxinfoComponent} from "../mxinfo/mxinfo.component";
import {MatCheckbox} from "@angular/material/checkbox";
import {MatDatepicker, MatDatepickerInput, MatDatepickerToggle} from "@angular/material/datepicker";
import {MatFormField, MatHint, MatLabel, MatSuffix} from "@angular/material/form-field";
import {MatInput} from "@angular/material/input";
import {MatOption} from "@angular/material/autocomplete";
import {MatSelect} from "@angular/material/select";
import {RouterOutlet} from "@angular/router";
import {DateAdapter, provideNativeDateAdapter} from '@angular/material/core';
import {AuthorizationService} from "../authorization.service";

@Component({
  selector: 'app-mx-adminpage',
  standalone: true,
  imports: [
    AsyncPipe,
    CommonModule,
    MatExpansionPanel,
    MatExpansionPanelTitle,
    MatExpansionPanelDescription,
    MatExpansionPanelHeader,
    FormsModule,
    MatCheckbox,
    MatDatepicker,
    MatDatepickerInput,
    MatDatepickerToggle,
    MatFormField,
    MatHint,
    MatInput,
    MatLabel,
    MatOption,
    MatSelect,
    MatSuffix,
    ReactiveFormsModule,
    RouterOutlet
  ],
  providers: [    provideNativeDateAdapter()
  ],
  templateUrl: './mx-adminpage.component.html',
  styleUrl: './mx-adminpage.component.css'
})
export class MxAdminpageComponent {
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
  ]
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
  protected approvedMorningExs: Observable<MorningExercise[]> = this.mx.getmxsbyfilter("is_approved%20=%20TRUE");
  protected pendingMorningExs: Observable<MorningExercise[]> = this.mx.getmxsbyfilter("is_approved%20=%20FALSE");
  protected Users = this.auth.getusers()
  filterform = new FormGroup({
    date:  new FormControl('',{nonNullable: true}),
    mingrade: new FormControl('', {nonNullable: true}),
    maxgrade: new FormControl('', {nonNullable: true}),
    datecreated: new FormControl('', {nonNullable: true}),
    owner: new FormControl('', {nonNullable: true}),
    title: new FormControl('', {nonNullable: true})
  })

  viewMX (title: string) {
    var morningExercise: Observable<MorningExercise[]> = this.mx.getmx(title);
    const dialogRef = this.dialog.open(MxinfoComponent, {
      width: '90vw',
      data: morningExercise,
    });
  }

  filter() {
    var datefilter = " "
    var gradefilter = " "
    var datecreatedfilter = " "
    var ownerfilter = " "
    var titlefilter = " "
    this.approvedMorningExs = this.mx.getmxsbyfilter("is_approved%20=%20TRUE" + datefilter + datecreatedfilter + ownerfilter + titlefilter + gradefilter) ;
    this.pendingMorningExs = this.mx.getmxsbyfilter("is_approved%20=%20FALSE" + datefilter + datecreatedfilter + ownerfilter + titlefilter + gradefilter);
  }
  constructor(
    private mx: MorningExService,
    private dialog: MatDialog,
    private auth: AuthorizationService
  ) {
  }
}
