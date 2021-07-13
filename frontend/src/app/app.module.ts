import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { HttpClientModule } from '@angular/common/http';
import { RouterModule } from '@angular/router';
import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { EmployeesComponent } from './components/employees/employees.component';
import { PersonsComponent } from './components/persons/persons.component';

const routes = [
  { path: '', redirectTo: 'home', pathMatch: 'full'},
  { path: 'employees', component: EmployeesComponent },
  { path: 'persons', component: PersonsComponent }
];

@NgModule({
  declarations: [
    AppComponent,
    EmployeesComponent,
    PersonsComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    HttpClientModule,
    RouterModule.forRoot(routes)
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
