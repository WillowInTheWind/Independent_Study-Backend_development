import { Injectable } from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {catchError} from "rxjs";
import {ERROR} from "@angular/compiler-cli/src/ngtsc/logging/src/console_logger";

@Injectable({
  providedIn: 'root'
})
export class MorningExService {
  async postMx (
          date:string,
          title: string,
          description: string,
          min_grade: number,
          max_grade: number,
          young_student_prep_instructions: string,
          is_available_in_day: boolean,
          required_tech_json: string,
          short_description: string,
          editors_json: string,
          is_approved: boolean
  )  {
    let morning_ex = {
      "date": date,
      "title": title,
      "description": description,
      "min_grade": min_grade,
      "max_grade": max_grade,
      "young_student_prep_instructions": young_student_prep_instructions,
      "is_available_in_day": is_available_in_day,
      "required_tech_json": required_tech_json,
      "short_description": short_description,
      "editors_json": editors_json
    };
    let outcome = true;
    let post = this.http.post("/api/morningexercises/create", morning_ex).subscribe({
        next: e => {
          console.log("jofkdhns");
          outcome = true;
          return outcome

        },
        error: e => {
          outcome = false;
          return outcome

        },
      }
  )
  return post
}
  editMx (   id:number,  date:string,
               title: string,
               description: string,
               min_grade: number,
               max_grade: number,
               young_student_prep_instructions: string,
               is_available_in_day: boolean,
               required_tech_json: string,
               short_description: string,
               editors_json: string,
               is_approved: boolean) {
    let morning_ex = {
      "id": id,
      "date": date,
      "title": title,
      "description": description,
      "min_grade": min_grade,
      "max_grade": max_grade,
      "young_student_prep_instructions": young_student_prep_instructions,
      "is_available_in_day": is_available_in_day,
      "required_tech_json": required_tech_json,
      "short_description": short_description,
      "editors_json": editors_json
    };

    let me = this.http.post("/api/morningexercises/edit", morning_ex).subscribe(
      date => {

      })
  }
   error () {
    window.alert("Something went wrong. Try again later")
     return false
  }
  getupcomingmxs () {
    return this.http.get<MorningExercise[]>("/api/external/upcoming",{params: {
        sub: "445c032762246a94cc96b0e2c74c5dffe5cc7ebc6ede8f5216aaa8eecc6f9d11"}})


  }
  deleteMx (id: number) {
    this.http.delete("/api/morningexercises/delete/"+ <string><unknown>id).subscribe(data => {
    })
  }
  getmx (title: string) {
    return this.http.get<MorningExercise[]>("/api/morningexercises/getbytitle",{params: {
        name: title}, withCredentials: true})
  }
  getmxsbyfilter(filter: string) {
    return this.http.get<MorningExercise[]>("/api/morningexercises/filterby",{params: {
      filter: filter}, withCredentials: true})
  }
  getusermxs() {

    return this.http.get<MorningExercise[]>("/api/morningexercises/mine",{withCredentials: true})
  }
  getusermxsbyname(name: string) {
    return this.http.get<MorningExercise[]>("/api/morningexercises/" +name)
  }
  approveMx(title: string) {
    this.http.post("/api/morningexercises/approve", {title}).subscribe(data => {
    })
  }
  revokeMx(title: string) {
    this.http.post("/api/morningexercises/revoke", {title}).subscribe(data => {
    })
  }
  getallmxs () {
    return this.http.get<MorningExercise[]>("/api/morningexercises", {withCredentials: true})
  }
  constructor(private http: HttpClient) { }
}

export interface MorningExercise{
  id: number,
  mx_index: number,
  date: string,
  owner: {
    sub: string,
    pictrue: string,
    email: string
    name: string
  },
  title: string,
  description: string,
  min_grade: number,
   max_grade: number,
   young_student_prep_instructions: string,
   is_available_in_day: boolean,
   required_tech_json: string[],
   short_description: string,
   editors_json: number[],
   is_approved: boolean
  // editors: Vec<GenericUser>
}


