import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders, HttpParams } from '@angular/common/http';
import { Observable, throwError } from 'rxjs';
import { retry, catchError } from 'rxjs/operators';
import { environment } from 'src/environments/environment';
import { IExample } from 'src/models/example.interface';

@Injectable({
  providedIn: 'root',
})
export class IotaStream {
  constructor(private http: HttpClient) {}

  writeDataToDatabase(className: string, body: any): Observable<Any> {
    const httpParams = new HttpParams();
    // httpParams = httpParams.append('address', JSON.stringify(address));
    // httpParams = httpParams.append('author', JSON.stringify(author));
    return this.http
      .post<any>(
        environment.SERVER_URL + '/classes/' + className,
        JSON.stringify(body),
        {
          headers: new HttpHeaders({
            'Content-Type': 'application/json',
            'X-Parse-Application-Id': environment.APP_ID,
            'X-Parse-REST-API-Key': environment.REST_API_KEY,
          }),
          params: httpParams,
        },
      )
      .pipe(retry(5), catchError(this.handleError));
  }

  fetchDataFromDatabase(className: string): Observable<any> {
    let httpParams = new HttpParams();
    httpParams = httpParams.append(
      'address',
      JSON.stringify(subscription.address),
    );
    httpParams = httpParams.append(
      'author',
      JSON.stringify(subscription.author),
    );
    return this.http
      .post<any>(environment.SERVER_URL + '/address/fetchAll', '', {
        headers: new HttpHeaders({
          'Content-Type': 'application/json',
        }),
        params: httpParams,
      })
      .pipe(retry(5), catchError(this.handleError));
  }

  handleError(error: {
    error: { message: string };
    status: any;
    message: any;
  }): Observable<never> {
    let errorMessage = '';
    if (error.error instanceof ErrorEvent) {
      // Get client-side error
      errorMessage = error.error.message;
    } else {
      // Get server-side error
      errorMessage = `Error Code: ${error.status}\nMessage: ${error.message}`;
    }
    window.alert(errorMessage);
    return throwError(errorMessage);
  }
}
